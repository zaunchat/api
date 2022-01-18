import { Request, Response, NextFunction } from '@tinyhttp/app'
import { Server as WebSocketServer } from 'ws'

export const ws = (server: WebSocketServer) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    const isSocket = req.headers.upgrade?.toLowerCase() === 'websocket'

    if (isSocket) {
        server.handleUpgrade(req, req.socket, Buffer.alloc(0), ws => server.emit('connection', ws, req))
    }

    next()
}