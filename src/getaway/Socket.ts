import WebSocket from 'ws'
import { Payload, WSCloseCodes } from './Constants'
import { Getaway } from './Getaway'
import Redis from 'ioredis'

export const DEFAULT_HEARTBEAT_TIME = 1000 * 42

export class Socket {
    heartbeatTimeout!: NodeJS.Timeout
    user_id!: string
    subscriptions = new Redis()
    constructor(public ws: WebSocket, public getaway: Getaway) {
        this.setHeartbeat()
        this.subscriptions.on('message', (topic: string, raw: string) => {
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

    async subscribe(...topics: (string | string[])[]): Promise<void> {
        this.subscriptions.disconnect()
        await this.subscriptions.subscribe(...topics.flat(4))
    }

    async unsubscribe(...topics: (string | string[])[]): Promise<void> {
        await this.subscriptions.unsubscribe(...topics.flat(4))
    }
}