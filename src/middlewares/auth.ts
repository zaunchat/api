import { Request, Response, NextFunction } from '@tinyhttp/app'
import { User } from '../structures'
import { HTTPError } from '../errors'


export const auth = (unauthorized: string[]) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (unauthorized.some((p) => req.path.includes(p))) {
        return next()
    }

    const token = req.headers['x-session-token']
    const userId = req.headers['x-session-id']

    const user = token && userId ? await User.findOne({
        _id: userId,
        deleted: false,
        verified: true
    }) : null

    if (!user?.sessions.some(session => session.token === token)) {
        throw new HTTPError('UNAUTHORIZED')
    }

    Object.defineProperty(req, 'user', {
        value: user
    })

    next()
}