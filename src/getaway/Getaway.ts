import WebSocket from 'ws'
import events from './events'
import { Socket } from './Socket'
import { WSEvents } from '../@types'
import { WSCodes, WSCloseCodes, Payload } from './Constants'

export class Getaway {
    server: WebSocket.Server
    connections = new Map<string, Socket>()
    constructor(options: WebSocket.ServerOptions = { noServer: true, maxPayload: 4096 }) {
        this.server = new WebSocket.Server(options)
        this.server.on('connection', this.onConnection.bind(this))
        this.server.on('error', this.onError.bind(this))
    }

    emit<T extends keyof WSEvents = keyof WSEvents>(event: T, data?: WSEvents[T]): void {
        console.debug(`
            Event: ${event},
            Data: ${JSON.stringify(data, null, 2)}
        `)
        // TODO: Emit event to connections/listeners
    }

    private async onConnection(_server: WebSocket.Server, _socket: WebSocket): Promise<void> {
        const socket = new Socket(_socket, this)

        try {
            socket.ws
                .on('close', this.onClose.bind(this))
                .on('message', (buffer) => this.onMessage(socket, buffer))

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

    private onClose(socket: WebSocket): void {
        socket.removeEventListener('message')
    }

    private onError(error: unknown): void {
        console.error(error)
    }
}
