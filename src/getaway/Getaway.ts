import WebSocket, { ServerOptions, WebSocketServer } from 'ws'
import { RateLimiterRedis } from 'rate-limiter-flexible'
import events from './events'
import { Socket } from './Socket'
import { WSCodes, WSCloseCodes, WSEvents, Payload } from './Constants'
import { createRedisConnection } from '../database/redis'
import { is, logger } from '../utils'
import ms from 'ms'

const HEARTBEAT_INTERVAL_MS = ms('45ms')

export class Getaway {
  redis = createRedisConnection()
  server: WebSocketServer
  connections = new Map<ID, Socket>()
  limiter = new RateLimiterRedis({
    storeClient: createRedisConnection(),
    points: 120,
    duration: ms('1 minute') / 1000,
    keyPrefix: 'ws'
  })

  constructor(options: ServerOptions) {
    this.server = new WebSocketServer(options)
    this.server.on('connection', this.onConnection.bind(this))
    this.server.on('error', this.onError.bind(this))
  }

  async publish<T extends keyof WSEvents>(topic: ID, event: T, data?: WSEvents[T]): Promise<void> {
    await this.redis.publish(topic, JSON.stringify({ event, data }))
  }

  async subscribe(targetId: ID, ...topics: ID[]): Promise<void> {
    await this.connections.get(targetId)?.subscribe(topics)
  }

  async unsubscribe(targetId: ID, ...topics: ID[]): Promise<void> {
    await this.connections.get(targetId)?.unsubscribe(topics)
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
          heartbeat_interval: HEARTBEAT_INTERVAL_MS
        }
      })
    } catch (error) {
      logger.error(error)
      socket.close(WSCloseCodes.UNKNOWN_ERROR)
    }
  }

  private async onMessage(socket: Socket, buffer: WebSocket.Data): Promise<void> {
    let limited = true

    await this.limiter.consume(socket.id).then(() => limited = false).catch(() => null)

    if (limited) {
      return socket.close(WSCloseCodes.RATE_LIMITED)
    }


    let payload: Payload

    // TODO: Add other encodings not only "json"

    try {
      payload = JSON.parse(String(buffer))

      if (Array.isArray(payload)) {
        throw 'Non-supported type'
      }

      if (is.empty(payload)) { // Ignore falsy values
        throw 'Empty payload'
      }
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
      logger.error(err)
    }
  }

  private async onClose(socket: Socket): Promise<void> {
    socket.subscriptions.disconnect()
  }

  private onError(error: unknown): void {
    logger.error(error)
  }
}
