import { Request, Response, NextFunction } from '@tinyhttp/app'
import { is } from '../utils'

interface JSONOptions {
  limit: number
}

const
  UNPROCESSABLE_ENTITY = 422,
  PAYLOAD_TOO_LARGE = 413,
  LENGTH_REQUIRED = 411

const isJSONMethod = (method?: string): boolean => !!method && (method === 'POST' || method === 'PUT' || method === 'PATCH')

export const json = ({ limit }: JSONOptions) => async (req: Request, res: Response, next: NextFunction): Promise<void> => {
  const status = (status: number) => void res.sendStatus(status)
  const header = (name: string) => req.headers[name]?.toString() ?? null

  if (isJSONMethod(req.method) && header('content-type') === 'application/json') {
    let length: number | string | null = header('content-length')

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

    // Ignore "__proto__" or "constructor" JSON attacks.
    if (is.suspicious(body)) return status(UNPROCESSABLE_ENTITY)

    try {
      req.body = JSON.parse(body)
    } catch {
      return status(UNPROCESSABLE_ENTITY)
    }

    if (is.nil(req.body)) return status(UNPROCESSABLE_ENTITY)
  }

  next()
}