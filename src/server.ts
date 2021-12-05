import { App as HTTPServer, extendMiddleware } from '@tinyhttp/app'
import { getaway } from './getaway'
import { register } from 'express-decorators'
import * as middlewares from './middlewares'
import * as controllers from './controllers'
import * as extensions from './extensions'


interface ServerOptions {
  port: number
  limits: Record<string, string>
  origin: string
}

class Server {
  readonly http = new HTTPServer({
    onError: middlewares.error(),
    applyExtensions: (req, res, next) => {
      extendMiddleware(this.http)(req, res, next)
      extensions.extend(req, res)
    },
    noMatchHandler: (_req, res) => res.sendStatus(404)
  })

  constructor(public readonly options: ServerOptions) { }

  async init() {

    // Security related
    this.http.use(middlewares.helmet())
    this.http.use('*', middlewares.cors({
      methods: ['GET', 'POST', 'DELETE', 'PATCH', 'PUT'],
      allowedHeaders: ['content-type', 'x-session-token']
    }))

    this.http.use('/auth', middlewares.cors({
      origin: this.options.origin,
      methods: ['GET', 'POST']
    }))

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

    const NON_AUTH_ROUTES = [
      '/getaway',
      '/auth/accounts'
    ], CAPTCHA_ROUTES = [
      '/auth/accounts/login',
      '/auth/accounts/register'
    ], KB_100 = 102400

    // Add other middlewares
    this.http
      .use(middlewares.validID())
      .use(middlewares.json({ parser: JSON.parse, limit: KB_100 }))
      .use(middlewares.captcha({ required: CAPTCHA_ROUTES }))
      .use(middlewares.auth({ ignored: NON_AUTH_ROUTES }))
      .use('/gateway', middlewares.ws(getaway.server))
  }

  listen(): Promise<void> {
    return new Promise((ok) => this.http.listen(this.options.port, ok))
  }
}


export default Server
