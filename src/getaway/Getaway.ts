import WebSocket from 'ws'
import Redis from 'ioredis'
import events from './events'
import { Socket } from './Socket'
import { WSCodes, WSCloseCodes, WSEvents, Payload } from './Constants'
import { PresenceStatus, User } from '../structures'
import config from '../../config'

export class Getaway {
    server: WebSocket.Server
    redis = new Redis(config.database.redis)
    connections = new Map<Snowflake, Socket>()
    constructor(options: WebSocket.ServerOptions = { noServer: true, maxPayload: 4096 }) {
        this.server = new WebSocket.Server(options)
        this.server.on('connection', this.onConnection.bind(this))
        this.server.on('error', this.onError.bind(this))
    }

    async publish<T extends keyof WSEvents = keyof WSEvents>(channel: Snowflake, event: T, data?: WSEvents[T]): Promise<void> {
        await this.redis.publish(channel, JSON.stringify({ event, data }))
    }

    async subscribe(targetId: Snowflake, ...topics: Snowflake[]): Promise<void> {
        await this.connections.get(targetId)?.subscribe(topics)
    }

    async unsubscribe(targetId: Snowflake, ...topics: Snowflake[]): Promise<void> {
        await this.connections.get(targetId)?.unsubscribe(topics)
    }

    private async onConnection(_socket: WebSocket): Promise<void> {
        const socket = new Socket(_socket, this)

        try {
            socket.ws
                .once('close', this.onClose.bind(this, socket))
                .on('message', (buffer) => this.onMessage(socket, buffer))
                .on('error', this.onError.bind(this))

            await socket.send({
                code: WSCodes.HELLO,
                data: {
                    heartbeat_interval: 1000 * 30
                }
            })
        } catch (error) {
            console.error(error)
            socket.close(WSCloseCodes.UNKNOWN_ERROR)
        }
    }

    private async onMessage(socket: Socket, buffer: WebSocket.Data): Promise<void> {
        let payload: Payload

        try {
            payload = JSON.parse(typeof buffer === 'string' ? buffer : '{invalid json}')
        } catch {
            return socket.close(WSCloseCodes.DECODE_ERROR)
        }

        const event = events[payload.code as keyof typeof events]

        if (!event) {
            return socket.close(WSCloseCodes.UNKNOWN_OPCODE)
        }

        try {
            await event(socket, payload)
        } catch (err) {
            socket.close(WSCloseCodes.UNKNOWN_ERROR)
            console.error(err)
        }
    }

    private async onClose(socket: Socket): Promise<void> {
        socket.subscriptions.disconnect()

        if (!socket.user_id) return

        this.connections.delete(socket.user_id)

        const user = await User.findOne({
            _id: socket.user_id,
            verified: true
        })

        if (!user) return

        const wasOnline = user.presence.status !== PresenceStatus.OFFLINE

        if (!wasOnline) return

        const newPresence = {
            ghostMode: user.presence.ghostMode,
            status: PresenceStatus.OFFLINE
        }

        await user.save({ presence: newPresence })
        await this.publish(socket.user_id, 'USER_UPDATE', {
            _id: socket.user_id,
            presence: newPresence
        })
    }

    private onError(error: unknown): void {
        console.error(error)
    }
}
