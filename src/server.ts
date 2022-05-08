import { App as HTTPServer } from '@tinyhttp/app'
import * as middlewares from './middlewares'
import * as controllers from './controllers'

interface InitServerOptions {
  origin: string
}

class Server {
  readonly http = new HTTPServer({
    onError: middlewares.error(),
    noMatchHandler: (_req, res) => res.sendStatus(404),
    settings: {
      networkExtensions: true,
      xPoweredBy: false
    }
  })

  async init({ origin }: InitServerOptions) {
    // Security related
    this.http
      .use(middlewares.helmet({ hidePoweredBy: false }))
      .use(middlewares.cors({
        methods: ['GET', 'POST', 'DELETE', 'PATCH', 'PUT'],
        headers: ['content-type', 'content-length', 'x-session-token']
      }))
      .use('/auth', middlewares.cors({ origin, methods: ['GET', 'POST'] }))
      .use(middlewares.rateLimit('20/5s', 'global'))

    controllers.mount(this.http)

    const NON_AUTH_ROUTES = [
      '/auth/accounts',
      '/ping'
    ]

    // Add other middlewares
    this.http
      .use(middlewares.validID())
      .use(middlewares.json({ limit: 102400 /* 100KB */ }))
      .use(middlewares.auth({ ignored: NON_AUTH_ROUTES }))
  }

  listen(port: number): Promise<void> {
    return new Promise((ok) => this.http.listen(port, ok))
  }
}


export default Server
