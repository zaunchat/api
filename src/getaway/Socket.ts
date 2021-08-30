import WebSocket from 'ws'
import { Payload, WSCloseCodes } from './Constants'
import { Getaway } from './Getaway'

export const DEFAULT_HEARTBEAT_TIME = 1000 * 42

export class Socket {
    heartbeatTimeout!: NodeJS.Timeout
    user_id!: string

    constructor(public ws: WebSocket, public getaway: Getaway) {
        this.setHeartbeat()
        this.ws.once('close', () => {
            if (this.user_id) getaway.connections.delete(this.user_id)
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

    subscribe(id: string) {}
    unsubscribe(id: string) {}

    listen(topic: unknown): void {
        console.log(topic)
        // TODO: Set listener
    }
}