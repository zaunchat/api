import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { CreateTextChannelSchema, TextChannel } from '../structures'
import { getaway } from '../server'
import { Permissions } from '../utils'


@web.basePath('/channels/:serverId')
export class ServerChannelController {
    @web.use()
    async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
        if (!req.user.servers.some(id => id === req.params.serverId)) {
            throw new HTTPError('UNKNOWN_SERVER')
        }
        next()
    }

    @web.get('/')
    async fetchChannels(req: Request, res: Response): Promise<void> {
        const channels = await TextChannel.find({
            serverId: req.params.serverId,
            deleted: false
        })
        res.json(channels)
    }


    @web.get('/:channelId')
    async fetchChannel(req: Request, res: Response): Promise<void> {
        const channel = await TextChannel.findOne({
            _id: req.params.channelId,
            serverId: req.params.serverId,
            deleted: false
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        res.json(channel)
    }

    @web.post('/')
    async createChannel(req: Request, res: Response): Promise<void> {
        req.check(CreateTextChannelSchema)

        const permissions = await Permissions.fetch(req.user, req.params.serverId)

        if (!permissions.has('MANAGE_CHANNELS')) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        const channel = await TextChannel.from({
            ...req.body,
            serverId: req.params.serverId
        }).save()

        getaway.publish(channel._id, 'CHANNEL_CREATE', channel)

        res.json(channel)
    }


    @web.route('delete', '/:channelId')
    async deleteChannel(req: Request, res: Response): Promise<void> {
        const channel = await TextChannel.findOne({
            deleted: false,
            _id: req.params.channelId
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        const permissions = await Permissions.fetch(req.user, req.params.serverId)

        if (!permissions.has('MANAGE_CHANNELS')) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        await channel.save({ deleted: true })

        getaway.publish(channel._id, 'CHANNEL_DELETE', {
            _id: channel._id,
            serverId: channel.serverId
        })

        res.ok()
    }
}