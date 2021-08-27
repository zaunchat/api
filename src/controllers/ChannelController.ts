import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel } from '../structures'
import { HTTPError } from '../errors'
import { Permissions } from '../utils'
import db from '../database'

@web.basePath('/channels')
export class ChannelController {
    @web.get('/:channelId')
    async fetchChannel(req: Request, res: Response): Promise<void> {
        const channel = await db.get(Channel).findOne({
            _id: req.params.channelId
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        const permissions = new Permissions('CHANNEL').for(channel).with(req.user)

        if (!permissions.has('VIEW_CHANNEL')) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        res.json(Channel.toObject(channel))
    }

    @web.del('/:channelId')
    async deleteChannel(req: Request, res: Response): Promise<void> {
        const channel = await db.get(Channel).findOne({
            _id: req.params.channelId
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        const permissions = new Permissions('CHANNEL').for(channel).with(req.user)

        if (!permissions.has('MANAGE_CHANNEL')) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        channel.deleted = true

        await db.get(Channel).persistAndFlush(channel)

        res.sendStatus(202)
    }
}