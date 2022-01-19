import { App as HTTPServer } from '@tinyhttp/app'
import { getaway } from './getaway'
import { logger } from './utils'
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
      .use('/getaway', middlewares.rateLimit('3/20s --ip', 'getaway'))


    for (const Controller of Object.values(controllers)) {
      const controller = new Controller()
      
      const { routes, guards } = controller.register(this.http)
      
      logger
        .log(`Loaded ${controller.name} with ${routes} route & ${guards} guard.`)
    }

    const NON_AUTH_ROUTES = [
      '/getaway',
      '/auth/accounts',
      '/ping'
    ]

    // Add other middlewares
    this.http
      .use(middlewares.validID())
      .use(middlewares.json({ limit: 102400 /* 100KB */ }))
      .use(middlewares.auth({ ignored: NON_AUTH_ROUTES }))
      .get('/gateway/:encoding?', middlewares.ws(getaway.server))
  }

  listen(port: number): Promise<void> {
    return new Promise((ok) => this.http.listen(port, ok))
  }
}


export default Server
