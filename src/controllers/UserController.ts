import { Response, Request } from '@tinyhttp/app'
import * as web from 'express-decorators'
import { HTTPError } from '../errors'
import { ChannelTypes, DMChannel, User } from '../structures'

@web.basePath('/users')
export default class UserController {
    @web.get('/:userId')
    async fetchUser(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        const user = await db.findOne(User, {
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

        if (!await db.count(User, { _id: userId })) {
            return void res.status(403).send(new HTTPError('UNKNOWN_USER'))
        }

        const exists = await db.findOne(DMChannel, {
            type: ChannelTypes.DM,
            $or: [{ userId }, { recipients: userId }]
        })

        if (exists) {
            return void res.json(DMChannel.toObject(exists))
        }

        const dm = DMChannel.from({
            userId: req.user._id,
            recipients: userId
        })

        await db.persistAndFlush(dm)

        res.json(DMChannel.toObject(dm))
    }
}