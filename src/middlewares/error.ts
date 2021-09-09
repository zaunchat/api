import { Request, Response } from '@tinyhttp/app'
import { HTTPError, CheckError } from '../errors'

export const error = () => async (err: Error, _req: Request, res: Response): Promise<void> => {
    if (err instanceof HTTPError || err instanceof CheckError) {
        res.status(err.status).json({ message: err.message })
    } else {
        console.error(err)
        res.sendStatus(502)
    }
}