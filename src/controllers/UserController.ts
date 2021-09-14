import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, RelationshipStatus, User } from '../structures'
import { HTTPError } from '../errors'


@web.basePath('/users')
export class UserController {
    @web.get('/:user_id')
    async fetchOne(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        const user = await User.findOne({
            _id: user_id === '@me' ? req.user._id : user_id
        }, { public: true })

        if (!user) {
            throw new HTTPError('UNKNOWN_USER')
        }

        res.json(user)
    }

    @web.get('/@me/relationships')
    async fetchRelationships(req: Request, res: Response): Promise<void> {
        const relationships = await User.find({
            _id: {
                $in: Array.from(req.user.relations.keys())
            }
        }, {
            public: true
        })

        res.json(relationships)
    }

    @web.get('/:user_id/dm')
    async openDM(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params as Record<string, ID>

        const target = await User.findOne({
            _id: user_id
        })

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        const exists = await Channel.findOne({
            type: ChannelTypes.DM,
            recipients: user_id
        })

        if (exists) {
            return void res.json(exists)
        }

        const dm = Channel.from({
            type: ChannelTypes.DM
        })

        dm.recipients.add(req.user, target)

        await dm.save()

        res.json(dm)
    }


    @web.post('/:user_id/friend')
    async friend(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user._id || user_id === '@me') {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne({
            _id: user_id
        }), user = req.user

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        const panding = target.relations.get(user._id) === RelationshipStatus.IN_COMING && user.relations.get(target._id) === RelationshipStatus.OUTGOING
        let status: RelationshipStatus

        if (panding) {
            status = RelationshipStatus.FRIEND
            target.relations.set(req.user._id, RelationshipStatus.FRIEND)
            user.relations.set(target._id, RelationshipStatus.FRIEND)
        } else {
            status = RelationshipStatus.IN_COMING
            target.relations.set(req.user._id, RelationshipStatus.OUTGOING)
            user.relations.set(target._id, RelationshipStatus.IN_COMING)
        }

        await Promise.all([target.save(), user.save()])

        res.json({ status })
    }

    @web.route('delete', '/:user_id/friend')
    async unfriend(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user._id || user_id === '@me') {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne({
            _id: user_id
        })

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        if (!req.user.relations.has(target._id)) {
            return void res.json({ status: null })
        }

        req.user.relations.delete(target._id)
        target.relations.delete(req.user._id)

        await Promise.all([
            target.save(),
            req.user.save()
        ])

        return void res.json({ status: null })
    }

    @web.post('/:user_id/block')
    async block(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user._id || user_id === '@me') {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne({
            _id: user_id
        })

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        const alreadyBlocked = req.user.relations.get(target._id) === RelationshipStatus.BLOCKED

        if (alreadyBlocked) {
            return void res.json({ status: RelationshipStatus.BLOCKED })
        }

        await Promise.all([
            req.user.save({
                relations: req.user.relations.set(target._id, RelationshipStatus.BLOCKED)
            }),
            target.save({
                relations: target.relations.set(req.user._id, RelationshipStatus.BLOCKED_OTHER)
            })
        ])

        res.json({ status: RelationshipStatus.BLOCKED })
    }

    @web.route('delete', '/:user_id')
    async unblock(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user._id || user_id === '@me') {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne({
            _id: user_id
        })

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        req.user.relations.delete(target._id)

        await req.user.save()

        res.json({ status: null })
    }
}