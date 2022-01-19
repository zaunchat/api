import WebSocket from 'ws'
import { Payload, WSCloseCodes, DEFAULT_HEARTBEAT_TIME, WSCodes } from './Constants'
import { getaway } from '../getaway'
import { is, logger, Permissions } from '../utils'
import { createRedisConnection } from '../database/redis'
import { setTimeout as sleep } from 'node:timers/promises'
import events from './events'
import erlpack from 'erlpack'
import { nanoid } from 'nanoid'

const NOTHING = void 0

export enum EncodingTypes {
  JSON,
  ETF
}

interface ClientOptions {
  encoding: EncodingTypes
}

export class Client {
  readonly id = nanoid(8)
  readonly user_id!: string
  readonly subscriptions = createRedisConnection()
  readonly permissions = new Map<string, Permissions>()
  readonly connections = new Set<WebSocket>()
  readonly timeouts = new WeakMap<WebSocket, NodeJS.Timeout>()
  constructor(socket: WebSocket, public readonly options: ClientOptions) {
    socket.onmessage = Client.onMessage.bind(null, this)
    socket.onclose = Client.onClose.bind(null, this)
    this.connections.add(socket)
    this.setHeartbeat(socket)
    this.subscriptions.on('message', Client.onSubscriptionMessage.bind(this))
  }

  get authenticated(): boolean {
    return !!this.user_id
  }

  setHeartbeat(socket: WebSocket): this {
    const timeoutId = this.timeouts.get(socket)

    if (timeoutId) clearTimeout(timeoutId)

    this.timeouts.set(socket, setTimeout(() => socket.close(WSCloseCodes.SESSION_TIMEOUT), DEFAULT_HEARTBEAT_TIME).unref())

    return this
  }

  send(payload: Payload, specificSocket?: WebSocket): Promise<void> {
    return new Promise((resolve, reject) => {
      const data = this.options.encoding === EncodingTypes.JSON ? JSON.stringify(payload) : erlpack.pack(payload)

      if (specificSocket) return specificSocket.send(data, (err) => {
        if (err) return reject(err)
        resolve()
      })

      let rejected = false

      for (const socket of this.connections) {
        if (rejected) break

        socket.send(data, (err) => {
          if (err) {
            reject(err)
            rejected = true
          }
        })
      }

      if (!rejected) resolve()
    })
  }

  async subscribe(topics: string[]): Promise<void> {
    await this.subscriptions.subscribe(...topics)
  }

  async unsubscribe(topics: string[]): Promise<void> {
    await this.subscriptions.unsubscribe(...topics)
  }

  private static async onSubscriptionMessage(this: Client, targetId: string, raw: string) {
    while (!this.authenticated) await sleep(10)

    const payload = JSON.parse(raw) as Payload & { data: any }

    switch (payload.event) {
      case 'SERVER_MEMBER_LEAVE':
        if (this.user_id === payload?.data?.id) {
          this.subscriptions.unsubscribe(payload.data.server_id)
        }
        break
      case 'SERVER_DELETE':
      case 'CHANNEL_DELETE':
        this.subscriptions.unsubscribe(targetId)
        break
    }

    const permissions = this.permissions.get(targetId) || new Permissions(Permissions.FLAGS.ADMINISTRATOR)

    // TODO: Add more events to check
    switch (payload.event) { // permissions check
      case 'MESSAGE_CREATE':
      case 'MESSAGE_UPDATE':
      case 'MESSAGE_DELETE':
        if (!permissions.has(Permissions.FLAGS.VIEW_CHANNEL)) return
        break
      default:
        break
    }

    await this.send(payload)
  }

  private static async onClose(client: Client, event: WebSocket.CloseEvent) {
    client.connections.delete(event.target)

    if (!client.connections.size) {
        client.subscriptions.disconnect()
        getaway.clients.delete(client.user_id)
    }
  }


  private static async onMessage(client: Client, { data, target: socket }: WebSocket.MessageEvent) {
    const isLimited = await getaway.limited(client)

    if (isLimited) return socket.close(WSCloseCodes.RATE_LIMITED)

    let payload: Payload

    try {
        if (client.options.encoding === EncodingTypes.JSON) {
            data = String(data)
            if (!is.suspicious(data)) payload = JSON.parse(data)
        } else if (client.options.encoding === EncodingTypes.ETF && Buffer.isBuffer(data)) {
            payload = erlpack.unpack(data)
        }
        if (!payload!) throw NOTHING
    } catch {
        return socket.close(WSCloseCodes.DECODE_ERROR)
    }

    if (!is.payload(payload)) return socket.close(WSCloseCodes.DECODE_ERROR)
    if (!(payload.code in events)) return socket.close(WSCloseCodes.UNKNOWN_OPCODE)
    if (!client.authenticated && payload.code !== WSCodes.AUTHENTICATE) return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)

    const event = events[payload.code]

    await event(client, socket, payload).catch((err) => {
        logger.error(err)
        socket.close(WSCloseCodes.UNKNOWN_ERROR)
    })
  }
}
