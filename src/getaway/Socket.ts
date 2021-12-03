import WebSocket from 'ws'
import { Payload, WSCloseCodes, WSEvents } from './Constants'
import { Getaway } from './Getaway'
import { Permissions } from '../utils'
import { createRedisConnection } from '../database/redis'
import { nanoid } from 'nanoid'
import { setTimeout as sleep } from 'timers/promises'
import ms from 'ms'

export const DEFAULT_HEARTBEAT_TIME = ms('45ms')

export class Socket {
  heartbeatTimeout?: NodeJS.Timeout
  id = nanoid(12)
  user_id!: ID
  subscriptions = createRedisConnection()
  permissions = new Map<string, Permissions>()
  isPermissionsCached = false
  constructor(public ws: WebSocket, public getaway: Getaway) {
    this.setHeartbeat()

    this.subscriptions.on('message', async (topic: ID, raw: string) => {
      // Don't made any operation unitl the permissions are fully cached.
      while (!this.isPermissionsCached) await sleep(25)

      const data = JSON.parse(raw)

      switch (data.event as keyof WSEvents) {
        case 'MEMBER_LEAVE_SERVER':
          if (this.user_id === data?.data?.id) {
            this.subscriptions.unsubscribe(data.data.server_id)
          }
          break
        case 'SERVER_DELETE':
        case 'CHANNEL_DELETE':
          this.subscriptions.unsubscribe(topic)
          break
      }

      const permissions = this.permissions.get(topic) || new Permissions(Permissions.FLAGS.ADMINISTRATOR)

      // TODO: Add more events to check
      switch (data.event as keyof WSEvents) { // permissions check
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

  async subscribe(topics: ID[]): Promise<void> {
    await this.subscriptions.subscribe(...topics)
  }

  async unsubscribe(topics: ID[]): Promise<void> {
    await this.subscriptions.unsubscribe(...topics)
  }
}
