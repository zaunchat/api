import { ServerOptions, WebSocketServer } from 'ws'
import { RateLimiterRedis } from 'rate-limiter-flexible'
import { Client, EncodingTypes } from './Client'
import { WSCodes, WSEvents, DEFAULT_HEARTBEAT_TIME } from './Constants'
import { createRedisConnection } from '@database/redis'
import { logger } from '@utils'
import ms from 'ms'

export class Getaway {
  readonly redis = createRedisConnection()
  readonly server: WebSocketServer
  readonly clients = new Map<string, Client>()
  readonly limiter = new RateLimiterRedis({
    storeClient: createRedisConnection(),
    points: 120,
    duration: ms('1 minute') / 1000,
    keyPrefix: 'ws'
  })

  constructor(options: ServerOptions) {
    this.server = new WebSocketServer(options)

    this.server.on('connection', async (socket, request) => {
      const encoding = request.url?.endsWith('?encoding=json') ? EncodingTypes.JSON
        : request.url?.endsWith('?encoding=etf') ? EncodingTypes.ETF : EncodingTypes.JSON

      const client = new Client(socket, {
        encoding
      })

      await client.send({
        code: WSCodes.HELLO,
        data: {
          heartbeat_interval: DEFAULT_HEARTBEAT_TIME
        }
      }, socket)
    })

    this.server.on('error', logger.error)
  }

  async publish<T extends keyof WSEvents>(topic: ID, event: T, data?: WSEvents[T]): Promise<void> {
    await this.redis.publish(topic, JSON.stringify({ event, data }))
  }

  async subscribe(targetId: ID, ...topics: ID[]): Promise<void> {
    await this.clients.get(targetId)?.subscribe(topics)
  }

  async unsubscribe(targetId: ID, ...topics: ID[]): Promise<void> {
    await this.clients.get(targetId)?.unsubscribe(topics)
  }

  async limited(client: Client): Promise<boolean> {
    let limited = true

    const key = client.user_id ?? client.id

    await this.limiter.consume(key).then(() => limited = false).catch(() => null)

    return limited
  }
}