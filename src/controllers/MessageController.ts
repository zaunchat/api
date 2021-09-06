import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { FilterQuery } from 'mikro-orm'
import { HTTPError } from '../errors'
import { CreateMessageSchema, Channel, Message } from '../structures'
import { Permissions } from '../utils'
import config from '../../config'


@web.basePath('/channels/:channelId/messages')
export class MessageController {
    @web.use()
    async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
        const channel = await Channel.findOne({
            _id: req.params.channelId
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        const permissions = await Permissions.fetch(req.user, null, channel)

        if (!permissions.has(Permissions.FLAGS.VIEW_CHANNEL)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        Object.defineProperty(req, 'permissions', {
            value: permissions
        })

        next()
    }

    @web.post('/')
    async sendMessage(req: Request, res: Response): Promise<void> {
        const permissions = (req as unknown as { permissions: Permissions }).permissions

        if (!permissions.has(Permissions.FLAGS.SEND_MESSAGES)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        req.check(CreateMessageSchema)

        const message = Message.from({
            ...req.body,
            authorId: req.user._id,
            channelId: req.params.channelId,
        })

        if (message.isEmpty()) {
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

        res.json(message)
    }

    @web.get('/')
    async fetchMessages(req: Request, res: Response): Promise<void> {
        const permissions = (req as unknown as { permissions: Permissions }).permissions

        if (!permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        let {
            limit = 50,
            before,
            after,
            around
        } = req.query

        limit = Number(limit)

        if (isNaN(limit) || limit > 100) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const options: FilterQuery<Message> = {
            channelId: req.params.channelId
        }

        if (typeof around === 'string') {
            options._id = {
                $or: [{
                    $gte: around
                }, {
                    $lt: around
                }]
            }
        }

        if (typeof after === 'string') {
            options._id = {
                $gt: after
            }
        }
        
        if (typeof before === 'string') {
            options._id = {
                $lt: before
            }
        }

        const messages = await Message.find(options, { limit })

        res.json(messages)
    }


    @web.get('/:messageId')
    async fetchMessage(req: Request, res: Response): Promise<void> {
        const permissions = (req as unknown as { permissions: Permissions }).permissions

        if (!permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId
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
            channelId: req.params.channelId
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
            channelId: req.params.channelId
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        if (message.authorId !== req.user._id) {
            const permissions = (req as unknown as { permissions: Permissions }).permissions
            if (!permissions.has(Permissions.FLAGS.MANAGE_MESSAGES)) {
                throw new HTTPError('MISSING_PERMISSIONS')
            }
        }

        await message.delete()

        res.ok()
    }
}