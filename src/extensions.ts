import { Request, Response } from '@tinyhttp/app'
import { SyncCheckFunction } from 'fastest-validator'
import { APIErrors, CheckError, HTTPError } from './errors'


export const extend = (req: Request, _res: Response) => {
    req.check = (checker: SyncCheckFunction): void => {
        const valid = checker(req.body)
        if (valid !== true) throw new CheckError(valid)
    }

    req.throw = (tag: keyof typeof APIErrors): void => {
        throw new HTTPError(tag)
    }
}
