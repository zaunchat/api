import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { DMChannel } from '../structures'
import { HTTPError } from '../errors'
import { Permissions } from '../utils'
import db from '../database'
import { getaway } from '../server'

@web.basePath('/channels')
export class ChannelController {
    @web.get('/:channelId')
    async fetchChannel(req: Request, res: Response): Promise<void> {
        const channel = await db.get(DMChannel).findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        const permissions = new Permissions('CHANNEL').for(channel).with(req.user)

        if (!permissions.has('VIEW_CHANNEL')) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        res.json(channel)
    }

    @web.route('delete', '/:channelId')
    async deleteChannel(req: Request, res: Response): Promise<void> {
        const channel = await db.get(DMChannel).findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        const permissions = new Permissions('CHANNEL').for(channel).with(req.user)

        if (!permissions.has('MANAGE_CHANNEL')) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        channel.deleted = true

        await db.get(DMChannel).persistAndFlush(channel)

        getaway.emit('CHANNEL_DELETE', { _id: channel._id })

        res.sendStatus(202)
    }
}