import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { Member, TextChannel } from '../structures'
import { getaway } from '../server'
import { Permissions, validator } from '../utils'


@web.basePath('/channels/:serverId')
export class ServerChannelController {
    checks = {
        createChannel: validator.compile({ name: { type: 'string' } })
    }

    @web.use()
    async fetchServerBeforeProcess(req: Request, res: Response, next: NextFunction): Promise<void> {
        const exists = await Member.findOne({
            _id: req.user._id,
            serverId: req.params.serverId
        })

        if (!exists) {
            return void res.status(404).send(new HTTPError('UNKNOWN_SERVER'))
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
            deleted: false
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        res.json(channel)
    }

    @web.post('/')
    async createChannel(req: Request, res: Response): Promise<void> {
        const valid = this.checks.createChannel(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const channel = TextChannel.from({
            ...req.body,
            serverId: req.params.serverId
        })

        const permissions = new Permissions('SERVER').for(channel).with(req.user)

        if (!permissions.has('MANAGE_CHANNELS')) {
            return void res.status(400).send(new HTTPError('MISSING_PERMISSIONS'))
        }

        await channel.save()

        res.json(channel)
    }


    @web.route('delete', '/:channelId')
    async deleteChannel(req: Request, res: Response): Promise<void> {
        const channel = await TextChannel.findOne({
            deleted: false,
            _id: req.params.channelId
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        const permissions = new Permissions('SERVER').for(channel).with(req.user)

        if (!permissions.has('MANAGE_CHANNELS')) {
            return void res.status(400).send(new HTTPError('MISSING_PERMISSIONS'))
        }

        await channel.save({ deleted: true })

        getaway.emit('CHANNEL_DELETE', {
            _id: channel._id,
            serverId: channel.serverId
        })

        res.sendStatus(202)
    }
}