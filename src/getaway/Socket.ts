import WebSocket from 'ws'
import { Payload, WSCloseCodes } from './Constants'
import { Getaway } from './Getaway'
import { Snowflake } from '../utils'
import Redis from 'ioredis'

export const DEFAULT_HEARTBEAT_TIME = 1000 * 42

export class Socket {
    heartbeatTimeout?: NodeJS.Timeout
    user_id!: Snowflake
    subscriptions = new Redis()
    constructor(public ws: WebSocket, public getaway: Getaway) {
        this.setHeartbeat()
        this.subscriptions.on('message', (topic: Snowflake, raw: string) => {
            const data = JSON.parse(raw) as Payload

            switch (data.event) {
                case 'SERVER_DELETE':
                case 'CHANNEL_DELETE':
                    this.subscriptions.unsubscribe(topic)
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