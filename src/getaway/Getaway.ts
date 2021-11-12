import WebSocket, { Server as WebSocketServer, ServerOptions } from 'ws'
import events from './events'
import { Socket } from './Socket'
import { WSCodes, WSCloseCodes, WSEvents, Payload } from './Constants'
import { PresenceStatus, User } from '../structures'
import { createRedisConnection } from '../database/redis'


export class Getaway {
    redis = createRedisConnection()
    server: WebSocketServer
    connections = new Map<ID, Socket>()
    constructor(options: ServerOptions = { noServer: true, maxPayload: 4096 }) {
        this.server = new WebSocketServer(options)
        this.server.on('connection', this.onConnection.bind(this))
        this.server.on('error', this.onError.bind(this))
    }

    async publish<T extends keyof WSEvents = keyof WSEvents>(channel: ID, event: T, data?: WSEvents[T]): Promise<void> {
        await this.redis.publish(channel, JSON.stringify({ event, data }))
    }

    async subscribe(targetId: ID, topics: ID[]): Promise<void> {
        await this.connections.get(targetId)?.subscribe(...topics)
    }

    async unsubscribe(targetId: ID, topics: ID[]): Promise<void> {
        await this.connections.get(targetId)?.unsubscribe(...topics)
    }

    private async onConnection(ws: WebSocket): Promise<void> {
        const socket = new Socket(ws, this)

        try {
            socket.ws
                .once('close', this.onClose.bind(this, socket))
                .on('message', this.onMessage.bind(this, socket))
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
            payload = JSON.parse(buffer.toString())
        } catch {
            return socket.close(WSCloseCodes.DECODE_ERROR)
        }

        if (typeof payload.code !== 'number') {
            return socket.close(WSCloseCodes.UNKNOWN_OPCODE)
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
            id: socket.user_id,
            verified: true
        })

        if (!user) return

        const wasOnline = user.presence.status !== PresenceStatus.OFFLINE

        if (!wasOnline) return

        const newPresence = {
            ghost_mode: user.presence.ghost_mode,
            status: PresenceStatus.OFFLINE
        }

        await user.save({ presence: newPresence })
    }

    private onError(error: unknown): void {
        console.error(error)
    }
}
