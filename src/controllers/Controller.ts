import { App, NextFunction, Request, Response, URLParams } from '@tinyhttp/app'
import { ParsedUrlQuery } from 'querystring'
import { APIErrors, HTTPError } from '@errors'
import { User } from '@structures'
import { Guard } from './decorators'

const METHODS = ['GET', 'POST', 'DELETE', 'PATCH', 'PUT', 'USE'] as const

export type Method = typeof METHODS[number]
export type Middleware = (ctx: Context, next: NextFunction) => Awaited<unknown>
export type BaseController = ReturnType<typeof Controller>
export type Next = NextFunction

export function Controller(path: string) {
  return class BaseController {
    readonly basePath = path
    static readonly guards: Guard[] = []

    get name(): string {
      return this.constructor.name
    }

    register(server: App): { routes: number, guards: number } {
      let routes = 0, guards = 0

      for (const guard of BaseController.guards) {
        server.use(this.basePath, guard)
        guards++
      }

      for (const route of this) {
        for (const guard of route.fn.guards ?? []) {
          server.use(route.path, guard)
          guards++
        }

        routes++

        server[route.method](route.path, async (req, res, next) => {
          const ctx = new Context(req, res)
          const response = await Promise.resolve(route.fn(ctx, next))

          if (typeof response === 'object') {
            res.json(response)
          } else if (typeof response === 'number') {
            res.sendStatus(response)
          } else {
            res.sendStatus(200) // OK
          }
        })
      }

      return { routes, guards }
    }

    *[Symbol.iterator]() {
      const basePath = this.basePath, regex = new RegExp(`^(${METHODS.join('|')})\\s`)

      for (const [opts, fn] of Object.entries(this)) {
        if (typeof fn !== 'function' || !regex.test(opts)) continue

        const [, method, path] = opts.split(regex) as [unknown, Method, string]

        const route = {
          method: method.toLowerCase() as Lowercase<Method>,
          path: basePath + path,
          fn
        }

        yield route
      }
    }
  }
}

export class Context {
  constructor(public readonly request: Request, public readonly response: Response) { }

  get body(): any {
    return this.request.body
  }

  get query(): ParsedUrlQuery {
    return this.request.query
  }

  get params(): URLParams {
    return this.request.params
  }

  get user(): User {
    return this.request.user
  }

  throw(tag: keyof typeof APIErrors): void {
    throw new HTTPError(tag)
  }

  header(name: string): string | null {
    return this.request.headers[name]?.toString() ?? null
  }
}


export * from './decorators'
