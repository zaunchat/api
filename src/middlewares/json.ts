import { Request, Response, NextFunction } from '@tinyhttp/app'

interface JSONOptions {
    parser: typeof JSON.parse,
    limit: number
}

export const json = ({ parser, limit }: JSONOptions) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (req.method && ['POST', 'PUT', 'PATCH'].includes(req.method)) {
        const length = Number(req.headers['content-length']) || 0

        if (length > limit) {
            return next('Request entity too large')
        }

        try {
            let body = ''
            for await (const chunk of req) body += chunk
            req.body = parser(body)
        } catch {
            return next('Invalid JSON body')
        }
    }
    
    next()
}