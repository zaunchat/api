import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { DMChannel, Group } from '../structures'
import { HTTPError } from '../errors'
import { getaway } from '../server'

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


    @web.get('/:channelId')
    async fetchChannel(req: Request, res: Response): Promise<void> {
        let channel: DMChannel | Group | null = await DMChannel.findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) channel = await Group.findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        res.json(channel)
    }

    @web.route('delete', '/:channelId')
    async deleteChannel(req: Request, res: Response): Promise<void> {
        let channel: DMChannel | Group | null = await DMChannel.findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) channel = await Group.findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        await channel.save({ deleted: true })

        getaway.emit('CHANNEL_DELETE', { _id: channel._id })

        res.sendStatus(202)
    }
}