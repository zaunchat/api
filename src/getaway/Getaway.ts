import WebSocket from 'ws'
import { App } from '@tinyhttp/app'
import { middlewares } from '../utils'
import { Socket } from './Socket'
import events from './events'

export class Getaway {
    ws: WebSocket.Server
    
    constructor(server: App, options: WebSocket.ServerOptions = { noServer: true, maxPayload: 4096 }) {
        server.use('/ws', middlewares.ws(this.ws = new WebSocket.Server(options)))
        this.ws.on('connection', this.onConnection.bind(this))
        this.ws.on('error', this.onError.bind(this))
    }

    private async onConnection(_server: WebSocket.Server, _socket: WebSocket): Promise<void> {
        const socket = new Socket(_socket)

        try {
            socket.ws
                .on('close', this.onClose.bind(this))
                .on('message', (buffer) => this.onMessage(socket, buffer))

            await socket.send({
                code: OPCODES.HELLO,
                data: {
                    heartbeat_interval: 1000 * 30
                }
            })
        } catch (error) {
            console.error(error)
            socket.close(WSCloseCodes.UNKNOWN_ERROR)
        }
    }

    private onMessage(socket: Socket, buffer: WebSocket.Data): void {
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

        event(socket, payload)
    }

    private onClose(socket: WebSocket): void {
        socket.removeEventListener('message')
    }

    private onError(error: unknown): void {
        console.error(error)
    }
}

export interface Payload {
    code: OPCODES
    event?: string
    data?: unknown
}

export enum OPCODES {
    HELLO,
    PING,
    PONG,
    AUTHENTICATE,
    AUTHENTICATED,
}


export enum WSCloseCodes {
    UNKNOWN_ERROR = 4000,
    UNKNOWN_OPCODE,
    DECODE_ERROR,
    NOT_AUTHENTICATED,
    AUTHENTICATED_FAILED,
    ALREADY_AUTHENTICATED,
    INVALID_SESSION,
    RATE_LIMITED,
    SESSION_TIMEOUT
}