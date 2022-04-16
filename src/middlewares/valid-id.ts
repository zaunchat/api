import { Request, Response, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { is } from '../utils'

export const validID = () => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {

  for (const key in req.params) {
    if (key.endsWith('_id') && !is.snowflake(req.params[key])) throw new HTTPError('INVALID_ID')
  }

  next()
}
