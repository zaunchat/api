import { Context, GenerateGuard, Controller, guards } from '@itchatt/controllers'
import { StringValue } from 'ms'
import { HTTPError, APIErrors } from '../errors'
import * as middlewares from '../middlewares'

Object.defineProperties(Context.prototype, {
  user: {
    get() { return this.request.user }
  },
  throw: { 
    value: (tag: keyof typeof APIErrors) => {
      throw new HTTPError(tag)
    }
  },
  header: {
    value: function (name: string) {
      return this.request.headers[name]?.toString() ?? null
    }
  }
})

export function Limit(limit: WithFlag<`${number}/${StringValue}`, 'ip'>) {
  return (target: typeof Controller) => {
    Object.defineProperty(target, 'GET /', { value: () => void 0 })
    guards.set(target.name + '/', middlewares.rateLimit(limit, target.name))
  }
}

export const Permission = {
  has: (...args: Parameters<typeof middlewares.permissions.has>) => GenerateGuard(middlewares.permissions.has(...args)),
  any: (...args: Parameters<typeof middlewares.permissions.any>) => GenerateGuard(middlewares.permissions.any(...args))
} as const

export const Check = (...args: Parameters<typeof middlewares.validate>) => GenerateGuard(middlewares.validate(...args))

export const Captcha = () => GenerateGuard(middlewares.captcha())

export { Context, Controller } from '@itchatt/controllers'
export { NextFunction as Next } from '@tinyhttp/app'