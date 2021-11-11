import { Request, Response, NextFunction } from '@tinyhttp/app'
import { RedisRateLimiter } from 'rolling-rate-limiter'
import { createRedisConnection } from '../database/redis'

const client = createRedisConnection()

interface RateLimitOptions {
  max: number
  interval: number
  onlyIP?: boolean
  message?: string
}

export const rateLimit = (options: RateLimitOptions, prefix: string): typeof middleware => {
  const limiter = new RedisRateLimiter({
    client,
    namespace: `rate-limit-${prefix}`,
    maxInInterval: options.max,
    interval: options.interval
  })

  if (!options.message) options.message = 'Too many requests, please try again later.'

  const middleware = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    let key = (req.ip || req.headers['x-forwarded-for'] || req.connection.remoteAddress) as string

    if (!options.onlyIP && req.user) key = req.user._id

    const info = await limiter.limitWithInfo(key)

    if (!info.blocked) {
      return next()
    }

    if (!res.headersSent) res
      .setHeader('X-RateLimit-Limit', options.max)
      .setHeader('X-RateLimit-Remaining', info.actionsRemaining)
      .setHeader('Retry-After', Math.ceil(info.millisecondsUntilAllowed / 1000))

    res
      .status(429)
      .json({
        message: options.message,
        retry_after: Math.ceil(info.millisecondsUntilAllowed / 1000)
      })
  }

  return middleware
}