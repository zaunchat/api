import { NextFunction, Request, Response } from '@tinyhttp/app'
import config from '../../config'
import WebSocket from 'ws'
import { Captcha } from './Captcha'
import db from '../database'
import { HTTPError } from '../errors'
import { User } from '../structures'

export const json = () => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
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
}

export const ws = (wss: WebSocket.Server) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    const isSocket = req.headers.upgrade?.split(',').some((s) => s.trim() === 'websocket')

    if (isSocket) {
        Object.defineProperty(req, 'ws', {
            value: new Promise((resolve) => wss.handleUpgrade(req, req.socket, Buffer.alloc(0), (ws) => {
                wss.emit('connection', ws, req)
                resolve(ws)
            }))
        })
    }

    next()
}

const NON_AUTH_ROUTES = ['login', 'register', 'verify'].map((r) => '/auth/' + r)


export const auth = () => async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    if (NON_AUTH_ROUTES.some((p) => req.path.includes(p))) {
        if (config('CAPTCHA').ENABLED) {
            const captchaChecked = req.body.captcha_key && await Captcha.check(req.body.captcha_key)
            if (!captchaChecked) {
                return void res.status(403).send(new HTTPError('FAILED_CAPTCHA'))
            }
        }
        
        return next()
    }

    const token = req.headers.authorization

    const user = token ? await db.get(User).findOne({
        sessions: { token }
    }) : null

    if (!user) {
        return void res.status(401).send(new HTTPError('UNAUTHORIZED'))
    }

    Object.defineProperty(req, 'user', {
        value: user
    })

    next()
}