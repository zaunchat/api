import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, CreateGroupSchema } from '../structures'
import { HTTPError } from '../errors'
import { Permissions } from '../utils'
import config from '../../config'


@web.basePath('/channels/@me')
export class ChannelController {
    @web.get('/')
    async fetchChannels(req: Request, res: Response): Promise<void> {
        const channels = await Channel.find({ recipients: req.user._id })
        res.json(channels)
    }

    @web.get('/:channelId')
    async fetchChannel(req: Request, res: Response): Promise<void> {
        const channel = await Channel.findOne({
            _id: req.params.channelId,
            recipients: req.user._id
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        res.json(channel)
    }

    @web.post('/')
    async createGroup(req: Request, res: Response): Promise<void> {
        req.check(CreateGroupSchema)

        const groupCount = await Channel.count({
            type: ChannelTypes.GROUP,
            recipients: req.user._id
        })

        if (groupCount >= config.limits.user.groups) {
            throw new HTTPError('MAXIMUM_GROUPS')
        }

        const group = await Channel.from({
            type: ChannelTypes.GROUP,
            name: req.body.name,
            ownerId: req.user._id,
            recipients: [req.user._id]
        }).save()

        res.json(group)
    }

    @web.post('/:groupId/:userId')
    async addMember(req: Request, res: Response): Promise<void> {
        const group = await Channel.findOne({
            type: ChannelTypes.GROUP,
            _id: req.params.groupId,
            recipients: req.user._id
        })

        if (!group) {
            throw new HTTPError('UNKNOWN_GROUP')
        }

        if (group.recipients.length >= config.limits.group.members) {
            throw new HTTPError('MAXIMUM_GROUP_MEMBERS')
        }

        if (group.recipients.includes(req.params.userId as Snowflake)) {
            throw new HTTPError('MISSING_ACCESS')
        }

        group.recipients.push(req.params.userId as Snowflake)

        await group.save()

        res.json(group)
    }

    @web.route('delete', '/:groupId/:userId')
    async kickMember(req: Request, res: Response): Promise<void> {
        const group = await Channel.findOne({
            type: ChannelTypes.GROUP,
            _id: req.params.groupId,
            recipients: req.user._id
        })

        if (!group) {
            throw new HTTPError('UNKNOWN_GROUP')
        }

        if (req.user._id === group.ownerId && req.user._id === req.params.userId) {
            throw new HTTPError('MISSING_ACCESS')
        }

        if (!group.recipients.includes(req.params.userId as Snowflake)) {
            throw new HTTPError('UNKNOWN_MEMBER')
        }

        const permissions = await Permissions.fetch(req.user, null, group)

        if (!permissions.has('KICK_MEMBERS')) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        await group.save({
            recipients: group.recipients.filter((id) => id !== req.params.userId)
        })

        res.ok()
    }

    @web.route('delete', '/:channelId')
    async deleteChannel(req: Request, res: Response): Promise<void> {
        const channel = await Channel.findOne({
            _id: req.params.channelId,
            recipients: req.user._id
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        if (channel.type === ChannelTypes.GROUP && channel.ownerId !== req.user._id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        await channel.delete()

        res.ok()
    }
}