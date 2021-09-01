import { App, Response } from '@tinyhttp/app'
import { middlewares } from './utils'
import { register } from 'express-decorators'
import * as Controllers from './controllers'
import { Getaway } from './getaway'


export const getaway = new Getaway()
export const server = new App({
    onError: middlewares.error()
})

server
    .use(middlewares.captcha(['/auth/login', '/auth/register']))
    .use(middlewares.json({ parse: JSON.parse }))
    .use(middlewares.auth(['/auth/verify', '/auth/check', '/ws']))
    .use('/ws', middlewares.ws(getaway.server))


for (const Controller of Object.values(Controllers)) {
    register(server, new Controller())
}


Object.defineProperty(Response.prototype, 'ok', {
    value: function () {
        (this as Response).sendStatus(202)
    }
})

Object.defineProperty(Request.prototype, 'check', {
    value: function (check: (x: unknown) => boolean) {
        const valid = check(this.body)

        if (valid !== true) {
            throw new Error(String(valid))
        }

        return true
    }
})

export default server