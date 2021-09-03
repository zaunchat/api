import { Request, Response, NextFunction } from '@tinyhttp/app'
import { RedisRateLimiter, RateLimiterOptions } from 'rolling-rate-limiter'
import Redis from 'ioredis'
import config from '../../config'

const client = new Redis(config.redis.uri && !config.redis.local ? config.redis.uri : void 0)


export const rateLimit = (options: Omit<RateLimiterOptions, 'client'>): typeof middleware => {
  const limiter = new RedisRateLimiter({
    ...options,
    namespace: 'ratelimit',
    client
  })

  const middleware = async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    const ip = (req.ip || req.headers['x-forwarded-for'] || req.connection.remoteAddress) as string
    const info = await limiter.limitWithInfo(ip)

    if (info.blocked) {
      if (!res.headersSent) {
        res.setHeader('X-RateLimit-Limit', options.maxInInterval)
        res.setHeader('X-RateLimit-Remaining', info.actionsRemaining)
        res.setHeader('Retry-After', Math.ceil(info.millisecondsUntilAllowed / 1000))
      }
      res.status(429).json({ message: 'Too many requests, please try again later.' })
    } else {
      next()
    }
  }

  return middleware
}