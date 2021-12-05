import { Request, Response, NextFunction } from '@tinyhttp/app'

interface CorsOptions {
  origin: string
  methods: string[]
  headers: string[]
  optionsSuccessStatus: number
}

export const cors = (options: Partial<CorsOptions>): typeof middleware => {
  const {
    origin = '*',
    methods = ['GET', 'HEAD', 'PUT', 'PATCH', 'POST', 'DELETE'],
    headers = ['content-type'],
    optionsSuccessStatus = 204
  } = options

  const middleware = (req: Request, res: Response, next: NextFunction) => {
    if (origin) res.setHeader('Access-Control-Allow-Origin', origin)
    if (methods) res.setHeader('Access-Control-Allow-Methods', methods)
    if (headers) res.setHeader('Access-Control-Allow-Headers', headers)
    if (req.method?.toUpperCase?.() === 'OPTIONS') {
      res.statusCode = optionsSuccessStatus
      res.setHeader('Content-Length', '0')
      res.end()
    } else {
      next?.()
    }
  }

  return middleware
}
