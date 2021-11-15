import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { CreateMessageSchema, Channel, Message } from '../structures'
import { is, Permissions } from '../utils'
import config from '../config'


@web.basePath('/channels/:channel_id/messages')
export class MessageController {
    @web.use()
    async authentication(req: Request, _res: Response, next: NextFunction): Promise<void> {
        const permissions = await Permissions.fetch({
            user: req.user,
            channel: req.params.channel_id as ID
        })

        if (!permissions.has(Permissions.FLAGS.VIEW_CHANNEL)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        Object.defineProperties(req, {
            permissions: {
                value: permissions
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
            author_id: req.user.id,
            channel_id: req.params.channel_id
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
            before,
            after,
            around
        } = req.query

        const limit = Number(req.query.limit ?? 50)

        if (isNaN(limit) || limit > 100 || limit < 0) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const filter = [`channel_id = ${req.params.id}`]

        if (is.snowflake(around)) {
            filter.push(`id >= ${around}`)
            filter.push(`id <= ${around}`)
        } else {
            if (is.snowflake(after)) filter.push(`id > ${after}`)
            if (is.snowflake(before)) filter.push(`id < ${before}`)
        }

        const messages = await Message.find(filter.join(' AND '), undefined, limit)

        res.json(messages)
    }


    @web.get('/:message_id')
    async fetchOne(req: Request, res: Response): Promise<void> {
        if (!req.permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        const { message_id, channel_id } = req.params
        const message = await Message.findOne(`id = ${message_id} AND channel_id = ${channel_id}`)

        res.json(message)
    }

    @web.patch('/:message_id')
    async edit(req: Request, res: Response): Promise<void> {
        req.check(CreateMessageSchema)

        const { message_id, channel_id } = req.params
        const message = await Message.findOne(`id = ${message_id} AND channel_id = ${channel_id}`)

        if (message.author_id !== req.user.id) {
            throw new HTTPError('CANNOT_EDIT_MESSAGE_BY_OTHER')
        }

        await message.update(req.body)

        res.json(message)
    }

    @web.route('delete', '/:message_id')
    async delete(req: Request, res: Response): Promise<void> {
        const { message_id, channel_id } = req.params
        const message = await Message.findOne(`id = ${message_id} AND channel_id = ${channel_id}`)

        if (message.author_id !== req.user.id && !req.permissions.has(Permissions.FLAGS.MANAGE_MESSAGES)) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        await message.delete()

        res.sendStatus(202)
    }
}