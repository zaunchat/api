import WebSocket from 'ws'
import { NextFunction, Request, Response } from '@tinyhttp/app'
import { HTTPError, CheckError } from '../errors'
import { User } from '../structures'
import config from '../../config'

export const json = ({ parse }: { parse: typeof JSON.parse }) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (req.method && ['POST', 'PUT', 'PATCH'].includes(req.method)) {
        try {
            let body = ''
            for await (const chunk of req) body += chunk
            req.body = parse(body.toString())
        } catch (e) {
            return next(e)
        }
    }
    next()
}

export const ws = (wss: WebSocket.Server) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    const isSocket = req.headers.upgrade?.toLowerCase() === 'websocket'

    if (isSocket) {
        wss.handleUpgrade(req, req.socket, Buffer.alloc(0), ws => wss.emit('connection', ws))
    }

    next()
}

export const auth = (unauthorized: string[]) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (unauthorized.some((p) => req.path.includes(p))) {
        return next()
    }

    const token = req.headers['x-session-token']
    const userId = req.headers['x-session-id']

    const user = token && userId ? await User.findOne({
        _id: userId,
        deleted: false,
        verified: true
    }) : null

    if (!user?.sessions.some(session => session.token === token)) {
        throw new HTTPError('UNAUTHORIZED')
    }

    Object.defineProperty(req, 'user', {
        value: user
    })

    next()
}

export const error = () => async (err: Error, _req: Request, res: Response, next?: NextFunction): Promise<void> => {
    if (err instanceof HTTPError || err instanceof CheckError) {
        res.status(err.status).send(err.message)
    } else {
        console.error(err)
        res.sendStatus(502)
    }
    next?.(err)
}


export const captcha = (requiredRoutes: string[]) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (config.captcha.enabled && requiredRoutes.some((p) => req.path.includes(p))) {
        if (!req.body.captcha_key) {
            throw new HTTPError('FAILED_CAPTCHA')
        }

        const payload = {
            secret: config.captcha.token,
            response: req.body.captcha_key,
            sitekey: config.captcha.key
        }

        const res = await fetch('https://hcaptcha.com/siteverify', {
            method: 'POST',
            body: JSON.stringify(payload),
        }).then((res) => res.json()).catch(() => false)


        if (!res?.success) {
            throw new HTTPError('FAILED_CAPTCHA')
        }
    }

    next()
}