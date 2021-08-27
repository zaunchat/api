import { NextFunction, Request, Response } from '@tinyhttp/app'
import WebSocket from 'ws'

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

export const ws = (
    options?: WebSocket.ServerOptions, 
    wss: WebSocket.Server = new WebSocket.Server({ ...options, noServer: true })
) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    const isWS = req.headers.upgrade?.split(',').some((s) => s.trim() === 'websocket')

    if (isWS) {
        Object.defineProperty(req, 'ws', {
            value: new Promise((resolve) => wss.handleUpgrade(req, req.socket, Buffer.alloc(0), (ws) => {
                wss.emit('connection', ws, req)
                resolve(ws)
            }))
        })
    }

    next()
}