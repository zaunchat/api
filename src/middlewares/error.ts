import { Request, Response, NextFunction } from '@tinyhttp/app'
import { HTTPError, CheckError } from '../errors'

export const error = () => async (err: Error, _req: Request, res: Response, next?: NextFunction): Promise<void> => {
    if (err instanceof HTTPError || err instanceof CheckError) {
        res.status(err.status).send(err.message)
    } else {
        console.error(err)
        res.sendStatus(502)
    }
    next?.(err)
}