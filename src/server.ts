import { App as HTTPServer, extendMiddleware } from '@tinyhttp/app'
import { getaway } from './getaway'
import { register } from 'express-decorators'
import * as middlewares from './middlewares'
import * as controllers from './controllers'
import * as extensions from './extensions'


interface ServerOptions {
  port: number
  limits: Record<string, string>
}

class Server {
  readonly http = new HTTPServer({
    onError: middlewares.error(),
    applyExtensions: (req, res, next) => {
      extendMiddleware(this.http)(req, res, next)
      extensions.extend(req, res)
    }
  })

  constructor(public readonly options: ServerOptions) { }

  async init() {
    this.http.use(middlewares.helmet())

    // Setup rate limiter
    for (const [route, opts] of Object.entries(this.options.limits)) {
      route === 'global' 
        ? this.http.use(middlewares.rateLimit(opts, 'global'))
        : this.http.use(route, middlewares.rateLimit(opts, route))
    }

    // Register Controllers
    for (const Controller of Object.values(controllers)) {
      register(this.http, new Controller())
    }

    // Add other middlewares
    this.http
      .use(middlewares.validID())
      .use(middlewares.json({ parser: JSON.parse, limit: 102400 /* 100KB */ }))
      .use(middlewares.captcha({ required: ['/auth/login', '/auth/register'] }))
      .use(middlewares.auth({ ignore: ['/auth/verify', '/gateway', '/test'] }))
      .use('/gateway', middlewares.ws(getaway.server))
  }

  listen(): Promise<void> {
    return new Promise((ok) => this.http.listen(this.options.port, ok))
  }
}


export default Server
