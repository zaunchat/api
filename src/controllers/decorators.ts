import { StringValue } from "ms"
import { BaseController, Middleware } from "./Controller"
import { Request, Response, NextFunction } from "@tinyhttp/app"
import * as middlewares from "@middlewares"

export type Guard = (request: Request, response: Response, next: NextFunction) => Awaited<unknown>

function AddGuard(guard: Guard) {
  return <T extends Middleware & { guards?: Guard[] }>(
    _target: Object,
    _key: string | symbol,
    descriptor: TypedPropertyDescriptor<T>
  ) => {
    const guards = descriptor.value!.guards || (descriptor.value!.guards = [])
    guards.push(guard)
    return descriptor
  }
}

export function Limit(limit: WithFlag<`${number}/${StringValue}`, 'ip'>) {
  return (target: BaseController) => {
    target.guards.push(middlewares.rateLimit(limit, target.name))
  }
}


export const Permission = {
  has: (...args: Parameters<typeof middlewares.permissions.has>) => AddGuard(middlewares.permissions.has(...args)),
  any: (...args: Parameters<typeof middlewares.permissions.any>) => AddGuard(middlewares.permissions.any(...args))
} as const


export function Check(...args: Parameters<typeof middlewares.validate>) {
  return AddGuard(middlewares.validate(...args))
}

export function Captcha() {
  return AddGuard(middlewares.captcha())
}