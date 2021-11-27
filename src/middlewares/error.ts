import { Request, Response } from '@tinyhttp/app'
import { HTTPError, CheckError } from '../errors'
import { logger } from '../utils'

export const error = () => async (err: unknown, _req: Request, res: Response): Promise<void> => {
  if (err instanceof HTTPError || err instanceof CheckError) {
    res.status(err.status).json({ message: err.message })
  } else {
    logger.error(err)
    res.sendStatus(502)
  }
}
