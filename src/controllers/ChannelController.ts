import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { ChannelTypes, DMChannel, Group } from '../structures'
import { HTTPError } from '../errors'
import { getaway } from '../server'
import { validator } from '../utils'
import config from '../../config'

@web.basePath('/channels/@me')
export class ChannelController {
    checks = {
        createGroup: validator.compile({
            name: { type: 'string' }
        })
    }

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
        const valid = this.checks.createGroup(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const groupCount = await Group.count({
            deleted: false,
            recipients: req.user._id
        })

        if (groupCount >= config.max.user.groups) {
            return void res.status(403).send(new HTTPError('MAXIMUM_GROUPS'))
        }

        const group = await Group.from({
            name: req.body.name,
            ownerId: req.user._id,
            recipients: [req.user._id]
        }).save()

        getaway.emit('CHANNEL_CREATE', group)

        res.json(group)
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

        if (channel.type === ChannelTypes.GROUP && channel.ownerId !== req.user._id && !channel.recipients.some((id) => id === req.user._id)) {
            return void res.status(400).send(new HTTPError('MISSING_ACCESS'))
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

        if (channel.type === ChannelTypes.GROUP && channel.ownerId !== req.user._id) {
            return void res.status(400).send(new HTTPError('MISSING_ACCESS'))
        }

        await channel.save({ deleted: true })

        getaway.emit('CHANNEL_DELETE', { _id: channel._id })

        res.sendStatus(202)
    }
}