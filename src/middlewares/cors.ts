import { Request, Response, NextFunction } from '@tinyhttp/app'

interface CorsOptions {
    origin: string
    methods: string[]
    headers: string[]
}

export const cors = ({
    origin = '*',
    methods = ['GET', 'PUT', 'PATCH', 'POST', 'DELETE'],
    headers = ['content-type', 'content-length', 'x-session-id', 'x-session-token']
}: Partial<CorsOptions> = {}) => async (_req: Request, res: Response, next: NextFunction): Promise<void> => {
    res.set('Access-Control-Allow-Origin', origin)
    res.setHeader('Access-Control-Allow-Methods', methods.join(', ').toUpperCase())
    res.setHeader('Access-Control-Allow-Headers', headers)
    next?.()
}