import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, CreateGroupSchema, User } from '../structures'
import { HTTPError } from '../errors'
import { Permissions } from '../utils'
import config from '../../config'


@web.basePath('/channels/@me')
export class ChannelController {
    @web.get('/')
    async fetchMany(req: Request, res: Response): Promise<void> {
        const channels = await Channel.find({ recipients: req.user._id })
        res.json(channels)
    }

    @web.get('/:channel_id')
    async fetchOne(req: Request, res: Response): Promise<void> {
        const channel = await Channel.findOne({
            _id: req.params.channel_id,
            recipients: req.user._id
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        res.json(channel)
    }

    @web.post('/')
    async create(req: Request, res: Response): Promise<void> {
        req.check(CreateGroupSchema)

        const groupCount = await Channel.count({
            type: ChannelTypes.GROUP,
            recipients: req.user._id
        })

        if (groupCount >= config.limits.user.groups) {
            throw new HTTPError('MAXIMUM_GROUPS')
        }

        const group = Channel.from({
            type: ChannelTypes.GROUP,
            name: req.body.name,
            owner: req.user
        })

        group.recipients.add(req.user)

        await group.save()

        res.json(group)
    }

    @web.post('/:group_id/:user_id')
    async add(req: Request, res: Response): Promise<void> {
        const user_id = req.params.user_id as ID

        const [group, target] = await Promise.all([
            Channel.findOne({
                type: ChannelTypes.GROUP,
                _id: req.params.group_id,
                recipients: req.user._id
            }),
            User.findOne({
                _id: user_id
            })
        ])

        if (!group) {
            throw new HTTPError('UNKNOWN_GROUP')
        }

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        if (group.recipients.length >= config.limits.group.members) {
            throw new HTTPError('MAXIMUM_GROUP_MEMBERS')
        }

        if (group.recipients.contains(target)) {
            throw new HTTPError('MISSING_ACCESS')
        }

        group.recipients.add(target)

        await group.save()

        res.json(group)
    }

    @web.route('delete', '/:group_id/:user_id')
    async kick(req: Request, res: Response): Promise<void> {
        const { user_id, group_id } = req.params

        const [group, target] = await Promise.all([
            Channel.findOne({
                type: ChannelTypes.GROUP,
                _id: group_id,
                recipients: req.user._id
            }),
            User.findOne({
                _id: user_id
            })
        ])

        if (!group) {
            throw new HTTPError('UNKNOWN_GROUP')
        }

        if (!target) {
            throw new HTTPError('UNKNOWN_USER')
        }

        if (req.user._id === group.owner._id && req.user._id === target._id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        if (!group.recipients.contains(target)) {
            throw new HTTPError('UNKNOWN_MEMBER')
        }

        const permissions = await Permissions.fetch(req.user, null, group)

        if (!permissions.has('KICK_MEMBERS')) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        group.recipients.remove(target)

        await group.save()

        res.sendStatus(202)
    }

    @web.route('delete', '/:channel_id')
    async delete(req: Request, res: Response): Promise<void> {
        const channel = await Channel.findOne({
            _id: req.params.channel_id,
            recipients: req.user._id
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        if (channel.type === ChannelTypes.GROUP && channel.owner._id !== req.user._id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        await channel.delete()

        res.sendStatus(202)
    }
}