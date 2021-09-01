import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { ChannelTypes, DMChannel, Group, CreateGroupSchema } from '../structures'
import { HTTPError } from '../errors'
import { getaway } from '../server'
import config from '../../config'


@web.basePath('/channels/@me')
export class ChannelController {
    @web.get('/')
    async fetchChannels(req: Request, res: Response): Promise<void> {
        const [dms, groups] = await Promise.all([
            DMChannel.find({
                recipients: req.user._id,
                deleted: false
            }),
            Group.find({
                recipients: req.user._id,
                deleted: false
            })
        ])
        res.json([...dms, ...groups])
    }

    @web.post('/')
    async createGroup(req: Request, res: Response): Promise<void> {
        req.check(CreateGroupSchema)

        const groupCount = await Group.count({
            deleted: false,
            recipients: req.user._id
        })

        if (groupCount >= config.limits.user.groups) {
            throw new HTTPError('MAXIMUM_GROUPS')
        }

        const group = await Group.from({
            name: req.body.name,
            ownerId: req.user._id,
            recipients: [req.user._id]
        }).save()

        await Promise.all(group.recipients.map((userId) => getaway.subscribe(userId, group._id)))

        getaway.publish(group._id, 'CHANNEL_CREATE', group)

        res.json(group)
    }


    @web.get('/:channelId')
    async fetchChannel(req: Request, res: Response): Promise<void> {
        const channel = await DMChannel.findOne({
            _id: req.params.channelId,
            recipients: req.user._id,
            deleted: false
        }) ?? await Group.findOne({
            _id: req.params.channelId,
            recipients: req.user._id,
            deleted: false
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        res.json(channel)
    }



    @web.route('delete', '/:channelId')
    async deleteChannel(req: Request, res: Response): Promise<void> {
        const channel = await DMChannel.findOne({
            _id: req.params.channelId,
            recipients: req.user._id,
            deleted: false
        }) ?? await Group.findOne({
            _id: req.params.channelId,
            recipients: req.user._id,
            deleted: false
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        if (channel.type === ChannelTypes.GROUP && channel.ownerId !== req.user._id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        await channel.save({ deleted: true })

        getaway.publish(channel._id, 'CHANNEL_DELETE', { _id: channel._id })

        res.ok()
    }
}