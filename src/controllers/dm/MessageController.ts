import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../../errors'
import { CreateMessageSchema, DMChannel, Message } from '../../structures'
import { BASE_CHANNEL_PATH } from '.'
import config from '../../../config'


@web.basePath(`${BASE_CHANNEL_PATH}/:channelId/messages`)
export class MessageController {
    @web.use()
    async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
        const dm = await DMChannel.findOne({
            _id: req.params.channelId,
            recipients: req.user._id
        })

        if (!dm) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        next()
    }

    @web.post('/')
    async sendMessage(req: Request, res: Response): Promise<void> {
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
        const limit = 50 // TODO: Add limit option
        const messages = await Message.find({
            channelId: req.params.channelId
        }, { limit })
        res.json(messages)
    }


    @web.get('/:messageId')
    async fetchMessage(req: Request, res: Response): Promise<void> {
        const message = await Message.findOne({
            _id: req.params.messageId,
            channelId: req.params.channelId,
            
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
            channelId: req.params.channelId,
            
        })

        if (!message) {
            throw new HTTPError('UNKNOWN_MESSAGE')
        }

        if (message.authorId !== req.user._id) {
            throw new HTTPError('MISSING_PERMISSIONS')
        }

        await message.delete()

        res.ok()
    }
}