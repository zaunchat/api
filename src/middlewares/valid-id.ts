import { Request, Response, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { is } from '../utils'

export const validID = () => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
  
  for (const [key, value] of Object.entries(req.params)) {
    if (key.endsWith('_id') && !is.snowflake(value)) throw new HTTPError('INVALID_ID')
  }
  
  next()
}
