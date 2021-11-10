import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { FilterQuery } from '@mikro-orm/core'
import { HTTPError } from '../errors'
import { CreateMessageSchema, Channel, Message } from '../structures'
import { is, Permissions } from '../utils'
import config from '../../config'


@web.basePath('/channels/:channel_id/messages')
export class MessageController {
    @web.use()
    async authentication(req: Request, _res: Response, next: NextFunction): Promise<void> {
        const channel = await Channel.findOne({
            _id: req.params.channel_id
        })

        if (!channel) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        const permissions = await Permissions.fetch(req.user, null, channel)

        if (!permissions.has(Permissions.FLAGS.VIEW_CHANNEL)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        Object.defineProperties(req, {
            permissions: {
                value: permissions
            },
            channel: {
                value: channel
            }
        })

        next()
    }

    @web.post('/')
    async send(req: Request, res: Response): Promise<void> {
        if (!req.permissions.has(Permissions.FLAGS.SEND_MESSAGES)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        req.check(CreateMessageSchema)

        const message = Message.from({
            ...req.body,
            author: req.user,
            channel: req.channel
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
    async fetchMany(req: Request, res: Response): Promise<void> {
        if (!req.permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        const {
            limit = 50,
            before,
            after,
            around
        } = req.query

        if (isNaN(Number(limit)) || limit > 100) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const options: FilterQuery<Message> = {
            channel: {
                _id: req.params.channel_id
            }
        }

        if (is.snowflake(around)) options._id = {
            $or: [{
                $gte: around
            }, {
                $lt: around
            }]
        }

        if (is.snowflake(after)) options._id = {
            $gt: after
        }

        if (is.snowflake(before)) options._id = {
            $lt: before
        }

        const messages = await Message.find(options, { limit: Number(limit) })

        res.json(messages)
    }


    @web.get('/:message_id')
    async fetchOne(req: Request, res: Response): Promise<void> {
        if (!req.permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        const message = await Message.findOne({
            _id: req.params.message_id,
            channel: {
                _id: req.params.channel_id
            }
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        res.json(message)
    }

    @web.patch('/:message_id')
    async edit(req: Request, res: Response): Promise<void> {
        req.check(CreateMessageSchema)

        const message = await Message.findOne({
            _id: req.params.message_id,
            channel: {
                _id: req.params.channel_id
            }
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        if (message.author._id !== req.user._id) {
            throw new HTTPError('CANNOT_EDIT_MESSAGE_BY_OTHER')
        }

        await message.save(req.body)

        res.json(message)
    }

    @web.route('delete', '/:message_id')
    async delete(req: Request, res: Response): Promise<void> {
        const message = await Message.findOne({
            _id: req.params.message_id,
            channel: {
                _id: req.params.channel_id
            }
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        if (message.author._id !== req.user._id && !req.permissions.has(Permissions.FLAGS.MANAGE_MESSAGES)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        await message.delete()

        res.ok()
    }
}