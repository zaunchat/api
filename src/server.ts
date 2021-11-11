import { App as HttpServer, extendMiddleware, Request, Response } from '@tinyhttp/app'
import { getaway } from './getaway'
import { register } from 'express-decorators'
import * as middlewares from './middlewares'
import * as controllers from './controllers'
import ms from 'ms'

interface ServerOptions {
    port: number
    limits: Record<string, string>
    extensions: (req: Request, res: Response) => void
}

class Server {
    readonly http = new HttpServer({
        onError: middlewares.error(),
        applyExtensions: (req, res, next) => {
            extendMiddleware(this.http)(req, res, next)
            this.options.extensions(req, res)
        }
    })

    constructor(public readonly options: ServerOptions) { }

    async init() {
        this.http.use(middlewares.helmet())

        // Setup rate limiter
        for (const [route, opts] of Object.entries(this.options.limits)) {
            const [max, interval, onlyIP] = opts.split(/\/|--/).map(s => s.trim())

            const options = {
                max: Number(max),
                interval: ms(interval),
                onlyIP: Boolean(onlyIP)
            }

            if (route === 'global') {
                this.http.use(middlewares.rateLimit(options, 'global'))
            } else {
                this.http.use(`/${route}`, middlewares.rateLimit(options, route))
            }
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

    async listen(): Promise<void> {
        return new Promise((resolve) => this.http.listen(this.options.port, resolve))
    }
}


export default Server