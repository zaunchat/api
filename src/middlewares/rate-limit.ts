import { Request, Response, NextFunction } from '@tinyhttp/app'
import { createRedisConnection } from '../database/redis'
import { RateLimiterRedis } from 'rate-limiter-flexible'


interface RateLimitOptions {
  max: number
  interval: number
  onlyIP?: boolean
  message?: string
}

export const rateLimit = (options: RateLimitOptions, prefix: string): typeof middleware => {
  const limiter = new RateLimiterRedis({
    storeClient: createRedisConnection(),
    points: options.max,
    duration: options.interval
  })

  if (!options.message) options.message = 'Too many requests, please try again later.'

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
        message: options.message,
        retry_after: Math.ceil(info.millisecondsUntilAllowed / 1000)
      })
  }

  return middleware
}