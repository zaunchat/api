import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import { Message, CreateMessageSchema, TextChannel } from '../structures'
import { getaway } from '../server'
import { Permissions } from '../utils'
import config from '../../config'


@web.basePath('/servers/:serverId/channels/:channelId/messages')
export class ServerMessageController {
	@web.use()
	async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
		if (!req.user.servers.some(id => id === req.params.serverId)) {
			throw new HTTPError('UNKNOWN_SERVER')
		}

		const channel = await TextChannel.findOne({
			_id: req.params.channelId,
			serverId: req.params._id,
			deleted: false
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

	@web.get('/')
	async fetchMessages(req: Request, res: Response): Promise<void> {
		const permissions = (req as unknown as { permissions: Permissions }).permissions

		if (!permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		const limit = 50 // TODO: Add limit option
		const messages = await Message.find({ channelId: req.params.channelId, deleted: false }, { limit })

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
			channelId: req.params.channelId,
			deleted: false
		})

		if (!message) {
			throw new HTTPError('UNKNOWN_MESSAGE')
		}

		res.json(message)
	}

	@web.post('/')
	async sendMessage(req: Request, res: Response): Promise<void> {
		req.check(CreateMessageSchema)

		const permissions = (req as unknown as { permissions: Permissions }).permissions

		if (!permissions.has(Permissions.FLAGS.SEND_MESSAGES)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		const message = Message.from({
			...req.body,
			authorId: req.user._id,
			channelId: req.params.channelId,
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
			if (!permissions.has(Permissions.FLAGS.MANAGE_MESSAGES)) throw new HTTPError('MISSING_PERMISSIONS')
		}

		await message.save({ deleted: true })

		getaway.publish(message.channelId, 'MESSAGE_DELETE', {
			_id: message._id,
			channelId: message.channelId
		})

		res.ok()
	}
}