import { Request, Response, NextFunction } from '@tinyhttp/app'

interface JSONOptions {
  parser: typeof JSON.parse
  limit: number
}

export const json = ({ parser, limit }: JSONOptions) => async (req: Request, res: Response, next: NextFunction): Promise<void> => {
  if (req.method && ['POST', 'PUT', 'PATCH'].includes(req.method)) {
    const contentType = req.headers['content-type']

    if (
      !contentType ||
      (typeof contentType === 'string' && contentType !== 'application/json') ||
      (Array.isArray(contentType) && !contentType.includes('application/json'))
    ) {
      return next()
    }

    const length = Number(req.headers['content-length']) || 0

    if (length > limit) {
      return next('Request entity too large')
    }

    try {
      let body = ''

      for await (const chunk of req) {
        body += chunk

        if (body.length > limit) {
          return void res.sendStatus(413)
        }
      }

      req.body = parser(body)
    } catch {
      return next('Invalid JSON body')
    }
  }

  next()
}
