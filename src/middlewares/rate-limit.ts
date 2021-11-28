import { Request, Response, NextFunction } from '@tinyhttp/app'
import { RateLimiterRedis } from 'rate-limiter-flexible'
import { createRedisConnection } from '../database/redis'
import ms from 'ms'

const storeClient = createRedisConnection()

export const rateLimit = (opts: string, prefix: string): typeof middleware => {
  const [max, interval, onlyIP] = opts.split(/\/|--/).map(s => s.trim())

  const options = {
    max: Number(max),
    interval: ms(interval as '1'),
    onlyIP: Boolean(onlyIP)
  }

  const limiter = new RateLimiterRedis({
    storeClient,
    points: options.max,
    duration: options.interval / 1000, // Per second(s)
    keyPrefix: prefix
  })


  const middleware = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    let key = (req.ip || req.headers['x-forwarded-for'] || req.connection.remoteAddress) as string
    let blocked = true

    if (!options.onlyIP && req.user) key = req.user.id

    const info = await limiter.consume(key).then(() => blocked = false).catch(res => res)

    if (!blocked) {
      return next()
    }

    if (!res.headersSent) res
      .setHeader("Retry-After", info.msBeforeNext / 1000)
      .setHeader("X-RateLimit-Limit", options.max)
      .setHeader("X-RateLimit-Remaining", info.remainingPoints)
      .setHeader("X-RateLimit-Reset", new Date(Date.now() + info.msBeforeNext).toString())


    res
      .status(429)
      .json({
        message: 'Too many requests, please try again later.',
        retry_after: info.msBeforeNext / 1000
      })
  }

  return middleware
}
