import { Request, Response, NextFunction } from '@tinyhttp/app'
import { is } from '../utils'

interface JSONOptions {
  limit: number
}

const
  UNPROCESSABLE_ENTITY = 422,
  PAYLOAD_TOO_LARGE = 413,
  LENGTH_REQUIRED = 411

export const parser = <T extends unknown>(input: string): T => {
  if (input === 'null') return null as T

  if (is.suspicious(input)) return JSON.parse(input, (key, value) => {
    if (key === '__proto__' || key === 'constructor') return
    return value
  })

  return JSON.parse(input)
}

const isJSONMethod = (method?: string): boolean => !!method && ['POST', 'PUT', 'PATCH'].includes(method)

export const json = ({ limit }: JSONOptions) => async (req: Request, res: Response, next: NextFunction): Promise<void> => {
  const status = (status: number) => void res.sendStatus(status)


  if (isJSONMethod(req.method) && req.header('content-type') === 'application/json') {
    let length: number | string | null = req.header('content-length')

    if (is.empty(length)) {
      return status(LENGTH_REQUIRED)
    }

    length = Number(length)

    if (length > limit) {
      return status(PAYLOAD_TOO_LARGE)
    }

    let body = ''

    for await (const chunk of req) {
      body += chunk
      if (body.length > limit) return status(PAYLOAD_TOO_LARGE)
    }

    try {
      req.body = parser(body)
    } catch {
      return status(UNPROCESSABLE_ENTITY)
    }

    if (is.nil(req.body)) return status(UNPROCESSABLE_ENTITY)
  }

  next()
}
