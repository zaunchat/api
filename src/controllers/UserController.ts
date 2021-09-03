import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { DMChannel, RelationshipStatus, User } from '../structures'
import { HTTPError } from '../errors'
import { getaway } from '../server'


@web.basePath('/users')
export class UserController {
    @web.get('/:userId')
    async fetchUser(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        const user = await User.findOne({
            _id: userId === '@me' ? req.user._id : userId,
            deleted: false
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        })

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
            },
            deleted: false
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        })

        res.json(relationships)
    }

    @web.get('/:userId/dm')
    async openDM(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        if (userId === req.user._id || userId === '@me') {
            return void res.status(403).json('You can\'t DM yourself')
        }

        if (!await User.findOne({ _id: userId })) {
            throw new HTTPError('UNKNOWN_USER')
        }

        const exists = await DMChannel.findOne({
            recipients: userId
        })

        if (exists) {
            return void res.json(exists)
        }

        const dm = await DMChannel.from({
            recipients: [userId as Snowflake, req.user._id]
        }).save()

        await Promise.all(dm.recipients.map((userId) => getaway.subscribe(userId, dm._id)))

        getaway.publish(dm._id, 'CHANNEL_CREATE', dm)

        res.json(dm)
    }


    @web.post('/:userId/friend')
    async friend(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        if (userId === req.user._id || userId === '@me') {
            return void res.status(403).json('You can\'t friend yourself')
        }

        const target = await User.findOne({
            _id: userId,
            deleted: false
        })

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        const panding = target.relations.get(req.user._id) === RelationshipStatus.IN_COMING && req.user.relations.get(target._id) === RelationshipStatus.OUTGOING
        let status: RelationshipStatus

        if (panding) {
            status = RelationshipStatus.FRIEND
            target.relations.set(req.user._id, RelationshipStatus.FRIEND)
            req.user.relations.set(target._id, RelationshipStatus.FRIEND)
        } else {
            status = RelationshipStatus.IN_COMING
            target.relations.set(req.user._id, RelationshipStatus.OUTGOING)
            req.user.relations.set(target._id, RelationshipStatus.IN_COMING)
        }

        await Promise.all([
            target.save(),
            req.user.save()
        ])

        res.send({ status })
    }

    @web.route('delete', '/:userId/friend')
    async unfriend(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        if (userId === req.user._id || userId === '@me') {
            return void res.status(403).json('You can\'t un-friend yourself')
        }

        const target = await User.findOne({
            _id: userId,
            deleted: false
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

    @web.post('/:userId/block')
    async block(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        if (userId === req.user._id || userId === '@me') {
            return void res.status(403).json('You can\'t block yourself')
        }

        const target = await User.findOne({
            _id: userId,
            deleted: false
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

    @web.route('delete', '/:userId')
    async unblock(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        if (userId === req.user._id || userId === '@me') {
            return void res.status(403).json('You can\'t unblock yourself')
        }

        const target = await User.findOne({
            _id: userId,
            deleted: false
        })

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        req.user.relations.delete(target._id)

        await req.user.save()

        res.json({ status: null })
    }
}