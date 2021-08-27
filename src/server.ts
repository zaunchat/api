import { App } from '@tinyhttp/app'
import config from '../config'
import { User } from './structures'
import { Captcha } from './utils/Captcha'
import { register } from 'express-decorators'
import { HTTPError } from './errors'
import db from './database'
import {
    AuthController,
    ChannelController,
    UserController,
    MessageController
} from './controllers'

const server = new App()
const NON_AUTH_ROUTES = ['login', 'register', 'verify'].map((r) => '/auth/' + r)


server
    .use(async (req, _res, next) => {
        if (req.method && ['POST', 'PUT', 'PATCH'].includes(req.method)) {
            try {
                let body = ''
                for await (const chunk of req) body += chunk
                req.body = JSON.parse(body.toString())
            } catch (e) {
               return next(e)
            }
        }
        next()
    })
    .use(async (req, res, next) => {
        if (NON_AUTH_ROUTES.some((p) => req.path.includes(p))) {
            if (config('CAPTCHA').ENABLED) {
                const captchaChecked = req.body.captcha_key && await Captcha.check(req.body.captcha_key)
                if (!captchaChecked) {
                    return res.status(403).send(new HTTPError('FIELD_CAPTCHA'))
                }
            }
            return next()
        }

        const token = req.headers.authorization

        const user = token ? await db.get(User).findOne({
            sessions: { token }
        }) : null

        if (!user) {
            return res.status(401).send(new HTTPError('UNAUTHORIZED'))
        }

        Object.defineProperty(req, 'user', {
            value: user
        })

        next()
    })

register(server, new AuthController())
register(server, new UserController())
register(server, new ChannelController())
register(server, new MessageController())

export default server