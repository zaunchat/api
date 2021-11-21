import { Request, Response } from '@tinyhttp/app'
import { SyncCheckFunction } from 'fastest-validator'
import { CheckError } from '../errors'

export const check = (req: Request, _res: Response) => {
  req.check = (checker: SyncCheckFunction): void => {
    const valid = checker(req.body)
    if (valid !== true) throw new CheckError(valid)
  }
}

