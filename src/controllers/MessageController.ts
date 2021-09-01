import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { CreateMessageSchema, DMChannel, Group, Message } from '../structures'
import { Permissions } from '../utils'
import { getaway } from '../server'
import config from '../../config'


@web.basePath('/channels/:channelId/messages')
export class MessageController {
    @web.use()
    async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
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

        const permissions = await Permissions.fetch(req.user, null, channel)

        Object.defineProperty(req, 'permissions', {
            value: permissions
        })

        next()
    }

    @web.post('/')
    async sendMessage(req: Request, res: Response): Promise<void> {
        req.check(CreateMessageSchema)

        const message = Message.from({
            authorId: req.user._id,
            channelId: req.params.channelId,
            ...req.body
        })

        if (!message.content?.length && !message.attachments.length) {
            throw new HTTPError('EMPTY_MESSAGE')
        }

        if ((message.content?.length ?? 0) > config.limits.message.length) {
            throw new HTTPError('MAXIMUM_MESSAGE_LENGTH')
        }

        if (message.replies.length > config.limits.message.replies) {
            throw new HTTPError('TOO_MANY_REPLIES')
        }

        if (message.attachments.length > config.limits.message.attachments) {
            throw new HTTPError('TOO_MANY_ATTACHMENTS')
        }

        await message.save()

        getaway.publish(message.channelId, 'MESSAGE_CREATE', message)

        res.json(message)
    }

    @web.get('/')
    async fetchMessages(req: Request, res: Response): Promise<void> {
        const permissions = (req as unknown as { permissions: Permissions }).permissions

        if (!permissions.has('READ_MESSAGE_HISTORY')) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        const limit = 50 // TODO: Add limit option
        const messages = await Message.find({ channelId: req.params.channelId, deleted: false }, { limit })
        res.json(messages)
    }


    @web.get('/:messageId')
    async fetchMessage(req: Request, res: Response): Promise<void> {
        const permissions = (req as unknown as { permissions: Permissions }).permissions

        if (!permissions.has('READ_MESSAGE_HISTORY')) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            deleted: false
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        res.json(message)
    }

    @web.patch('/:messageId')
    async editMessage(req: Request, res: Response): Promise<void> {
        req.check(CreateMessageSchema)

        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            deleted: false
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        if (message.authorId !== req.user._id) {
            throw new HTTPError('CANNOT_EDIT_MESSAGE_BY_OTHER')
        }

        await message.save(req.body)

        res.json(message)
    }

    @web.route('delete', '/:messageId')
    async deleteMessage(req: Request, res: Response): Promise<void> {
        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            deleted: false
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        if (message.authorId !== req.user._id) {
            const permissions = (req as unknown as { permissions: Permissions }).permissions
            if (!permissions.has('MANAGE_MESSAGES')) throw new HTTPError('MISSING_PERMISSIONS')
        }

        await message.save({ deleted: true })

        getaway.publish(message.channelId, 'MESSAGE_DELETE', {
            _id: message._id,
            channelId: message.channelId
        })

        res.ok()
    }
}