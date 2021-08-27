import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { Permissions } from '../utils'
import db from '../database'
import { Channel, Message } from '../structures'
import Validator from 'fastest-validator'
import { wrap } from 'mikro-orm'
import { getaway } from '../server'
import config from '../../config'

const validator = new Validator()

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
        const channel = await db.get(Channel).findOne({
            _id: req.params.channelId,
            deleted: false
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        const permissions = new Permissions('CHANNEL')
            .for(channel)
            .with(req.user)

        if (!permissions.has(['READ_MESSAGES', 'VIEW_CHANNEL'])) {
            return void res.status(403).send(new HTTPError('MISSING_PERMISSIONS'))
        }

        Object.defineProperty(req, 'channel', {
            value: channel
        })

        next()
    }

    @web.post('/')
    async sendMessage(req: Request, res: Response): Promise<void> {
        const channel = (req as unknown as { channel: Channel }).channel

        const permissions = new Permissions('CHANNEL')
            .for(channel)
            .with(req.user)

        if (!permissions.has('SEND_MESSAGES')) {
            return void res.status(403).send(new HTTPError('MISSING_PERMISSIONS'))
        }

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

        if (message.attachments.length && !permissions.has('UPLOAD_FILES')) {
            return void res.status(400).send(new HTTPError('MISSING_PERMISSIONS'))
        }

        if ((message.content?.length ?? 0) > config('MAX').MESSAGE_LENGTH) {
            return void res.status(400).send(new HTTPError('MAXIMUM_MESSAGE_LENGTH'))
        }

        if (message.replies.length > 5) {
            return void res.status(400).send(new HTTPError('TOO_MANY_REPLIES'))
        }

        if (message.attachments.length > 5) {
            return void res.status(400).send(new HTTPError('TOO_MANY_ATTACHMENTS'))
        }


        await db.get(Message).persistAndFlush(message)

        // FIXME: message has undefined id.
        // We should fetch the message from database
        // Or find another solution.
        getaway.emit('MESSAGE_CREATE', message)

        res.sendStatus(202)
    }

    @web.get('/')
    async fetchMessages(req: Request, res: Response): Promise<void> {
        const messages = await db.get(Message).find({
            channelId: req.params.channelId,
            deleted: false
        }, { limit: 100 })
        res.json(messages)
    }


    @web.get('/:messageId')
    async fetchMessage(req: Request, res: Response): Promise<void> {
        const message = await db.get(Message).findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId
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

        const message = await db.get(Message).findOne({
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

        await db.get(Message).persistAndFlush(wrap(message).assign(req.body))

        res.sendStatus(202)
    }

    @web.route('delete', '/:messageId')
    async deleteMessage(req: Request, res: Response): Promise<void> {
        const channel = (req as unknown as { channel: Channel }).channel

        const permissions = new Permissions('CHANNEL')
            .for(channel)
            .with(req.user)

        const message = await db.get(Message).findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            deleted: false
        })

        if (!message) {
            return void res.status(404).send(new HTTPError('UNKNOWN_MESSAGE'))
        }

        if (!permissions.has('MANAGE_MESSAGES') && message.authorId !== req.user._id) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        message.deleted = true

        await db.get(Message).persistAndFlush(message)

        getaway.emit('MESSAGE_DELETE', {
            _id: message._id,
            channelId: message.channelId
        })

        res.sendStatus(202)
    }
}