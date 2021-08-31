import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { Permissions, validator } from '../utils'
import { Channel, DMChannel, Message } from '../structures'
import { getaway } from '../server'
import config from '../../config'


@web.basePath('/channels/:channelId/messages')
export class MessageController {
    checks = {
        editMessage: validator.compile({
            content: { type: 'string' }
        }),
        sendMessage: validator.compile({
            content: { type: 'string' }
        })
    }

    @web.use()
    async fetchChannelBeforeProcess(req: Request, res: Response, next: NextFunction): Promise<void> {
        const channel = await Channel.findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        // if (!channel.recipients.some((id) => id === req.user._id)) {
        //     return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        // }

        Object.defineProperty(req, 'channel', {
            value: channel
        })

        next()
    }

    @web.post('/')
    async sendMessage(req: Request, res: Response): Promise<void> {
        const valid = this.checks.editMessage(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const message = Message.from({
            authorId: req.user._id,
            channelId: req.params.channelId,
            ...req.body
        })

        if (!message.content?.length && !message.attachments.length) {
            return void res.status(400).send(new HTTPError('EMPTY_MESSAGE'))
        }

        if ((message.content?.length ?? 0) > config.max.message.length) {
            return void res.status(400).send(new HTTPError('MAXIMUM_MESSAGE_LENGTH'))
        }

        if (message.replies.length > config.max.message.replies) {
            return void res.status(400).send(new HTTPError('TOO_MANY_REPLIES'))
        }

        if (message.attachments.length > config.max.message.attachments) {
            return void res.status(400).send(new HTTPError('TOO_MANY_ATTACHMENTS'))
        }

        await message.save()

        getaway.emit(message.channelId, 'MESSAGE_CREATE', message)

        res.sendStatus(202)
    }

    @web.get('/')
    async fetchMessages(req: Request, res: Response): Promise<void> {
        const limit = 50 // TODO: Add limit option
        const messages = await Message.find({ channelId: req.params.channelId, deleted: false }, { limit })
        res.json(messages)
    }


    @web.get('/:messageId')
    async fetchMessage(req: Request, res: Response): Promise<void> {
        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            deleted: false
        })

        if (!message) {
            return void res.status(404).send(new HTTPError('UNKNOWN_MESSAGE'))
        }

        res.json(message)
    }

    @web.patch('/:messageId')
    async editMessage(req: Request, res: Response): Promise<void> {
        const valid = this.checks.editMessage(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            deleted: false
        })

        if (!message) {
            return void res.status(404).send(new HTTPError('UNKNOWN_MESSAGE'))
        }

        if (message.authorId !== req.user._id) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        await message.save(req.body)

        res.sendStatus(202)
    }

    @web.route('delete', '/:messageId')
    async deleteMessage(req: Request, res: Response): Promise<void> {
        const channel = (req as unknown as { channel: Channel }).channel

        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            deleted: false
        })

        if (!message) {
            return void res.status(404).send(new HTTPError('UNKNOWN_MESSAGE'))
        }

        const permissions = new Permissions('CHANNEL')
            .for(channel)
            .with(req.user)

        if (!permissions.has('MANAGE_MESSAGES') && message.authorId !== req.user._id) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        await message.save({ deleted: true })
        
        getaway.emit(message.channelId, 'MESSAGE_DELETE', {
            _id: message._id,
            channelId: message.channelId
        })

        res.sendStatus(202)
    }
}