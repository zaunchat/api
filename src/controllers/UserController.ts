import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, PUBLIC_USER_PROPS, RelationshipStatus, User } from '../structures'
import { HTTPError } from '../errors'


@web.basePath('/users')
export class UserController {
    @web.get('/:user_id')
    async fetchOne(req: Request, res: Response): Promise<void> {
        const user = await User.findOne(`id = ${req.params.user_id}`)
        res.json(user)
    }

    @web.get('/@me/relationships')
    async fetchRelationships(req: Request, res: Response): Promise<void> {
        const relationships = await User.find(`id IN (${[...req.user.relations.keys()]})`, PUBLIC_USER_PROPS)
        res.json(relationships)
    }

    @web.get('/:user_id/dm')
    async openDM(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params as Record<string, ID>
        const target = await User.findOne(`id = ${user_id}`)
        const exists = await Channel.findOne(`type = ${ChannelTypes.DM} AND recipients::jsonb ? ${user_id}`).catch(() => null)

        if (exists) {
            return void res.json(exists)
        }

        const dm = Channel.from({
            type: ChannelTypes.DM,
            recipients: [req.user.id, target.id]
        })

        await dm.save()

        res.json(dm)
    }


    @web.post('/:user_id/friend')
    async friend(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user.id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne(`id = ${user_id}`)
        const user = req.user

        switch (user.relations.find((r) => r.id === target.id)?.status) {
            case RelationshipStatus.FRIEND: throw new HTTPError('ALREADY_FRIENDS')
            case RelationshipStatus.OUTGOING: throw new HTTPError('ALREADY_SENT_REQUEST')
            case RelationshipStatus.BLOCKED: throw new HTTPError('BLOCKED')
            case RelationshipStatus.BLOCKED_OTHER: throw new HTTPError('BLOCKED_BY_OTHER')
            case RelationshipStatus.IN_COMING: {
                // User = Friend
                // Target = Friend
                // TODO: Unknown
                break
            }
            default: {
                // User = Outgoing
                // Target = IN_COMING
                break
            }
        }

        await Promise.all([
            target.update({}),
            user.update({})
        ])

        res.json({ status: null })
    }

    @web.route('delete', '/:user_id/friend')
    async unfriend(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user.id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne(`id = ${user_id}`)

        if (!req.user.relations.some(r => r.id === target.id)) {
            return void res.json({ status: null })
        }

        await Promise.all([
            target.update({
                relations: target.relations.filter((c) => !(c.status === RelationshipStatus.BLOCKED && c.id === req.user.id))
            }),
            req.user.update({
                relations: req.user.relations.filter((c) => !(c.status === RelationshipStatus.FRIEND && c.id === target.id))
            })
        ])

        return void res.json({ status: null })
    }

    @web.post('/:user_id/block')
    async block(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user.id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne(`id = ${user_id}`)
        const alreadyBlocked = req.user.relations.get(target.id) === RelationshipStatus.BLOCKED

        if (alreadyBlocked) {
            return void res.json({ status: RelationshipStatus.BLOCKED })
        }

        await Promise.all([
            req.user.update({
                relations: req.user.relations.set(target.id, RelationshipStatus.BLOCKED)
            }),
            target.update({
                relations: target.relations.set(req.user.id, RelationshipStatus.BLOCKED_OTHER)
            })
        ])

        res.json({ status: RelationshipStatus.BLOCKED })
    }

    @web.route('delete', '/:user_id')
    async unblock(req: Request, res: Response): Promise<void> {
        const { user_id } = req.params

        if (user_id === req.user.id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const target = await User.findOne(`id = ${user_id}`)

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        await req.user.update({
            relations: req.user.relations.filter((c) => c.status === RelationshipStatus.BLOCKED && c.id === target.id)
        })

        res.json({ status: null })
    }
}