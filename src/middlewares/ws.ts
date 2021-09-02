import { Request, Response, NextFunction } from '@tinyhttp/app'
import WebSocket from 'ws'

export const ws = (wss: WebSocket.Server) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    const isSocket = req.headers.upgrade?.toLowerCase() === 'websocket'

    if (isSocket) {
        wss.handleUpgrade(req, req.socket, Buffer.alloc(0), ws => wss.emit('connection', ws))
    }

    next()
}