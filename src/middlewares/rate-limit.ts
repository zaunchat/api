import { Request, Response, NextFunction } from '@tinyhttp/app'
import { RedisRateLimiter } from 'rolling-rate-limiter'
import Redis from 'ioredis'
import config from '../../config'

const client = new Redis(config.redis.uri && !config.redis.local ? config.redis.uri : void 0)

interface RateLimitOptions {
  max: number
  interval: number
  onlyIP?: boolean
}

export const rateLimit = (options: RateLimitOptions, prefix: string): typeof middleware => {
  const limiter = new RedisRateLimiter({
    client,
    namespace: `rate-limit-${prefix}`,
    maxInInterval: options.max,
    interval: options.interval
  })

  const middleware = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    let key = (req.ip || req.headers['x-forwarded-for'] || req.connection.remoteAddress) as string

    if (!options.onlyIP && req.user) key = req.user._id

    const info = await limiter.limitWithInfo(key)

    if (info.blocked) {
      if (!res.headersSent) {
        res.setHeader('X-RateLimit-Limit', options.max)
        res.setHeader('X-RateLimit-Remaining', info.actionsRemaining)
        res.setHeader('Retry-After', Math.ceil(info.millisecondsUntilAllowed / 1000))
      }

      res.status(429).json({
        message: 'Too many requests, please try again later.',
        retry_after: Math.ceil(info.millisecondsUntilAllowed / 1000)
      })
    } else {
      next()
    }
  }

  return middleware
}