import WebSocket from 'ws'
import Redis from 'ioredis'
import { Payload, WSCloseCodes, WSEvents } from './Constants'
import { Getaway } from './Getaway'
import { Permissions } from '../utils'
import config from '../../config'

export const DEFAULT_HEARTBEAT_TIME = 1000 * 42

export class Socket {
    heartbeatTimeout?: NodeJS.Timeout
    user_id!: Snowflake
    subscriptions = new Redis(config.redis.uri && !config.redis.local ? config.redis.uri : void 0)
    constructor(public ws: WebSocket, public getaway: Getaway) {
        this.setHeartbeat()

        this.subscriptions.on('message', (topic: Snowflake, raw: string) => {
            const data = JSON.parse(raw)

            switch (data.event as keyof WSEvents) {
                case 'MEMBER_LEAVE_SERVER':
                    if (this.user_id === data?.data?._id) {
                        this.subscriptions.unsubscribe(data.data.serverId)
                    }
                    break
                case 'SERVER_DELETE':
                case 'CHANNEL_DELETE':
                    this.subscriptions.unsubscribe(topic)
                    break
            }

            const permissions = new Permissions(Permissions.FLAGS.ADMINISTRATOR) // TODO: Fetch permission

            // TODO: Add more events to check
            switch (data.event) { // permissions check
                case 'MESSAGE_CREATE':
                case 'MESSAGE_UPDATE':
                case 'MESSAGE_DELETE':
                    if (!permissions.has(Permissions.FLAGS.VIEW_CHANNEL)) return
                    break
                default:
                    break
            }

            this.ws.send(raw)
        })
    }

    setHeartbeat(time = DEFAULT_HEARTBEAT_TIME): this {
        if (this.heartbeatTimeout) clearTimeout(this.heartbeatTimeout)

        this.heartbeatTimeout = setTimeout(() => this.close(WSCloseCodes.SESSION_TIMEOUT), time).unref()

        return this
    }

    send(data: Payload): Promise<void> {
        return new Promise((resolve, reject) => this.ws.send(JSON.stringify(data), (err) => {
            if (err) return reject(err)
            resolve()
        }))
    }

    close(code: WSCloseCodes = WSCloseCodes.UNKNOWN_ERROR): void {
        this.ws.close(code)
    }

    async subscribe(topics: Snowflake | Snowflake[] | Snowflake[][]): Promise<void> {
        if (Array.isArray(topics)) {
            await this.subscriptions.subscribe(...topics.flat(4))
        } else {
            await this.subscriptions.subscribe(topics)
        }
    }

    async unsubscribe(topics: Snowflake | Snowflake[] | Snowflake[][]): Promise<void> {
        if (Array.isArray(topics)) {
            await this.subscriptions.unsubscribe(...topics.flat(4))
        } else {
            await this.subscriptions.unsubscribe(topics)
        }
    }
}