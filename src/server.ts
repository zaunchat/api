import { App } from '@tinyhttp/app'
import { IncomingMessage as Request, ServerResponse as Response } from 'http'
import { register } from 'express-decorators'
import { Getaway } from './getaway'
import { CheckError } from './errors'
import * as middlewares from './middlewares'
import * as Controllers from './controllers'


export const getaway = new Getaway()
export const server = new App({
    onError: middlewares.error()
})

server
    .use(middlewares.helmet())
    .use(middlewares.rateLimit({ interval: 1000, maxInInterval: 50 }))
    .use(middlewares.json({ parser: JSON.parse }))
    .use(middlewares.captcha(['/auth/login', '/auth/register']))
    .use(middlewares.auth(['/auth/verify', '/auth/check', '/ws']))
    .use('/ws', middlewares.ws(getaway.server))


for (const Controller of Object.values(Controllers)) {
    if (typeof Controller !== 'string') register(server, new Controller())
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