import { Response, Request } from '@tinyhttp/app'
import * as web from 'express-decorators'
import db from '../database'
import { HTTPError } from '../errors'
import { ChannelTypes, DMChannel, User } from '../structures'

@web.basePath('/users')
export class UserController {
    @web.get('/:userId')
    async fetchUser(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        const user = await db.get(User).findOne({
            _id: userId === '@me' ? req.user._id : userId
        })

        if (!user) {
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        res.json(User.toObject(user))
    }

    @web.get('/:userId/dm')
    async openDM(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        if (userId === req.user._id) {
            return void res.status(403).json('You can\'t DM yourself')
        }

        if (!await db.get(User).count({ _id: userId })) {
            return void res.status(403).send(new HTTPError('UNKNOWN_USER'))
        }

        const exists = await db.get(DMChannel).findOne({
            type: ChannelTypes.DM,
            $or: [{ userId }, { recipients: userId }]
        })

        if (exists) {
            return void res.json(DMChannel.toObject(exists))
        }

        const dm = DMChannel.from({
            recipients: [userId, req.user._id]
        })

        await db.get(DMChannel).persistAndFlush(dm)

        res.json(DMChannel.toObject(dm))
    }
}