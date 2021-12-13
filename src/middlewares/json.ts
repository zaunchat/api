import { Request, Response, NextFunction } from '@tinyhttp/app'
import { is } from '../utils'

interface JSONOptions {
  limit: number
}

export const parser = <T extends unknown>(input: string): T => {
  if (input === 'null') return null as T

  if (is.suspicious(input)) return JSON.parse(input, (key, value) => {
    if (key === '__proto__' || key === 'constructor') return
    return value
  })

  return JSON.parse(input)
}


export const json = ({ limit }: JSONOptions) => async (req: Request, res: Response, next: NextFunction): Promise<void> => {
  if (req.method && ['POST', 'PUT', 'PATCH'].includes(req.method)) {

    if (req.header('content-type') !== 'application/json') {
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
        if (body.length > limit) return void res.sendStatus(413)
      }

      req.body = parser(body)

      if (is.empty(req.body)) throw 'Invalid'
    } catch {
      return next('Invalid JSON body')
    }
  }

  next()
}
