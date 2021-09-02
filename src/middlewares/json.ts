import { Request, Response, NextFunction } from '@tinyhttp/app'

export const json = ({ parse }: { parse: typeof JSON.parse }) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (req.method && ['POST', 'PUT', 'PATCH'].includes(req.method)) {
        try {
            let body = ''
            for await (const chunk of req) body += chunk
            req.body = parse(body)
        } catch (e) {
            return next(e)
        }
    }
    next()
}