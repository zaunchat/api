import { Request, Response, NextFunction } from '@tinyhttp/app'
import { User } from '../structures'
import { HTTPError } from '../errors'


interface AuthOptions {
    ignore: `/${string}`[]
}

export const auth = (options: AuthOptions) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (options.ignore.some((p) => req.path.includes(p))) {
        return next()
    }

    const token = req.headers['x-session-token']
    const user = token ? await User.findOne({
        sessions: { token },
        verified: true
    }) : null

    if (!user) {
        throw new HTTPError('UNAUTHORIZED')
    }

    Object.defineProperty(req, 'user', {
        value: user
    })

    next()
}