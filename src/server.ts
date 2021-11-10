import { App as Server } from '@tinyhttp/app'
import { IncomingMessage as Request, ServerResponse as Response, STATUS_CODES } from 'http'
import { register } from 'express-decorators'
import { Getaway } from './getaway'
import { CheckError } from './errors'
import * as middlewares from './middlewares'
import * as Controllers from './controllers'
import config from '../config'
import ms from 'ms'


export const getaway = new Getaway()
export const server = new Server({
    onError: middlewares.error()
}).use(middlewares.helmet())


for (const [route, opts] of Object.entries(config.routes)) {
    const [max, interval, onlyIP] = opts.split(/\/|--/).map(s => s.trim())

    const options = {
        max: Number(max),
        interval: ms(interval),
        onlyIP: Boolean(onlyIP)
    }

    if (route === 'global') {
        server.use(middlewares.rateLimit(options, 'global'))
    } else {
        server.use(`/${route}`, middlewares.rateLimit(options, route))
    }
}


server
    .use(middlewares.validID())
    .use(middlewares.json({ parser: JSON.parse, limit: 102400 /* 100KB  */ }))
    .use(middlewares.captcha({ required: ['/auth/login', '/auth/register'] }))
    .use(middlewares.auth({ ignore: ['/auth/verify', '/gateway', '/test'] }))
    .use('/gateway', middlewares.ws(getaway.server))


for (const Controller of Object.values(Controllers)) {
    register(server, new Controller())
}


Object.defineProperty(Response.prototype, 'ok', {
    value: function (status = 202) {
        const res = this as Response
        res.statusCode = status
        res.setHeader('Content-Type', 'text/plain')
        res.end(STATUS_CODES[status], 'utf8')
    }
})

Object.defineProperty(Request.prototype, 'check', {
    value: function (check: (x: unknown) => boolean) {
        const valid = check(this.body)

        if (valid !== true) {
            throw new CheckError(valid)
        }

        return true
    }
})

export default server