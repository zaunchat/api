import { Request, Response, NextFunction } from '@tinyhttp/app'
import { User } from '../structures'
import { HTTPError } from '../errors'

interface AuthOptions {
  ignored: string[]
}

export const auth = (options: AuthOptions) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
  if (options.ignored.some((p) => req.path.includes(p))) {
    return next()
  }

  const token = req.headers['x-session-token']

  if (!token || Array.isArray(token)) {
    throw new HTTPError('MISSING_HEADER')
  }

  const user = await User.fetchByToken(token)

  if (!user) {
    throw new HTTPError('UNAUTHORIZED')
  }

  Object.defineProperty(req, 'user', {
    value: user
  })

  next()
}
