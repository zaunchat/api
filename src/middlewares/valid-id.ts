import { Request, Response, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { Snowflake } from '../utils'


const keys = [
    'channel',
    'user',
    'group',
    'server',
    'message',
    'member',
    'role'
].map(key => key + '_id')

export const validID = () => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {

    if (req.params) {
        for (const key of keys.filter(k => k in req.params)) {
            if (!Snowflake.is(req.params[key])) {
                throw new HTTPError('INVALID_ID')
            }
        }
    }

    
    next()
}