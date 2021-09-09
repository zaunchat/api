import { App as Server } from '@tinyhttp/app'
import { IncomingMessage as Request, ServerResponse as Response } from 'http'
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
    .use(middlewares.json({ parser: JSON.parse }))
    .use(middlewares.captcha(['/auth/login', '/auth/register']))
    .use(middlewares.auth(['/auth/verify', '/auth/check', '/gateway']))
    .use('/gateway', middlewares.ws(getaway.server))


for (const Controller of Object.values(Controllers)) {
    register(server, new Controller())
}


Object.defineProperty(Response.prototype, 'ok', {
    value: function () {
        this.sendStatus(202)
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