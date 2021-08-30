import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { getaway } from '../server'
import { ChannelTypes, DMChannel, RelationshipStatus, User } from '../structures'
import db from '../database'

@web.basePath('/users')
export class UserController {
    @web.get('/:userId')
    async fetchUser(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        const user = await db.get(User).findOne({
            _id: userId === '@me' ? req.user._id : userId,
            deleted: false
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        })

        if (!user) {
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        res.json(user)
    }

    @web.get('/:userId/friends')
    async fetchFriends(req: Request, res: Response): Promise<void> {
        if (req.params.userId !== '@me') {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        const friends = await db.get(User).find({
            _id: {
                $in: req.user.relations.filter((r) => r.status === RelationshipStatus.FRIEND).map((r) => r.id)
            },
            deleted: false
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        })

        res.json(friends)
    }

    @web.get('/:userId/blocked')
    async fetchBlocked(req: Request, res: Response): Promise<void> {
        if (req.params.userId !== '@me') {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        const friends = await db.get(User).find({
            _id: {
                $in: req.user.relations.filter((r) => r.status === RelationshipStatus.BLOCKED).map((r) => r.id)
            },
            deleted: false
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        })

        res.json(friends)
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
            recipients: userId
        })

        if (exists) {
            return void res.json(exists)
        }

        const dm = DMChannel.from({
            recipients: [userId, req.user._id]
        })

        await db.get(DMChannel).persistAndFlush(dm)

        getaway.emit('CHANNEL_CREATE', dm)

        res.json(dm)
    }
}