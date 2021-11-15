import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, CreateGroupSchema, Group, User } from '../structures'
import { HTTPError } from '../errors'
import { Permissions } from '../utils'
import config from '../config'


@web.basePath('/channels/@me')
export class ChannelController {
    @web.get('/')
    async fetchMany(req: Request, res: Response): Promise<void> {
        const channels = await Channel.find(`recipients::jsonb ? ${req.user.id}`)
        res.json(channels)
    }

    @web.get('/:channel_id')
    async fetchOne(req: Request, res: Response): Promise<void> {
        const channel = await Channel.findOne(`id = ${req.params.channel_id} AND recipients::jsonb ? ${req.user.id}`)
        res.json(channel)
    }

    @web.post('/')
    async create(req: Request, res: Response): Promise<void> {
        req.check(CreateGroupSchema)

        const groupCount = await Channel.count(`type = ${ChannelTypes.GROUP} AND recipients::jsonb ? ${req.user.id}`)

        if (groupCount >= config.limits.user.groups) {
            throw new HTTPError('MAXIMUM_GROUPS')
        }

        const group = Channel.from({
            type: ChannelTypes.GROUP,
            name: req.body.name,
            owner_id: req.user.id,
            recipients: [req.user.id]
        })

        await group.save()

        res.json(group)
    }

    @web.post('/:group_id/:user_id')
    async add(req: Request, res: Response): Promise<void> {
        const { user_id, group_id } = req.params

        const [group, target] = await Promise.all([
            Channel.findOne(`id = ${group_id} AND type = ${ChannelTypes.GROUP} AND recipients::jsonb ? ${req.user.id}`) as Promise<Group>,
            User.findOne(`id = ${user_id}`)
        ])

        if (group.recipients.length >= config.limits.group.members) {
            throw new HTTPError('MAXIMUM_GROUP_MEMBERS')
        }

        if (group.recipients.includes(target.id)) {
            throw new HTTPError('MISSING_ACCESS')
        }

        await group.update({
            recipients: [...group.recipients, target.id]
        })

        res.json(group)
    }

    @web.route('delete', '/:group_id/:user_id')
    async kick(req: Request, res: Response): Promise<void> {
        const { user_id, group_id } = req.params

        const [group, target] = await Promise.all([
            Channel.findOne(`id = ${group_id} AND type = ${ChannelTypes.GROUP} AND recipients::jsonb ? ${req.user.id}`) as Promise<Group>,
            User.findOne(`id = ${user_id}`)
        ])

        if (req.user.id === group.owner_id && req.user.id === target.id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        if (!group.recipients.includes(target.id)) {
            throw new HTTPError('UNKNOWN_MEMBER')
        }

        const permissions = await Permissions.fetch({
            user: req.user,
            channel: group
        })

        if (!permissions.has('KICK_MEMBERS')) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        await group.update({
            recipients: group.recipients.filter((id) => id !== target.id)
        })

        res.sendStatus(202)
    }

    @web.route('delete', '/:channel_id')
    async delete(req: Request, res: Response): Promise<void> {
        const channel = await Channel.findOne(`id = ${req.params.channel_id} AND type = ${ChannelTypes.GROUP} AND recipients::jsonb ? ${req.user.id}`)

        if (channel.type === ChannelTypes.GROUP && channel.owner_id !== req.user.id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        await channel.delete()

        res.sendStatus(202)
    }
}