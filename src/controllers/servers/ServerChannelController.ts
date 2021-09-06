import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../../errors'
import { Channel, CreateTextChannelSchema, ChannelTypes } from '../../structures'
import { Permissions } from '../../utils'


@web.basePath('/channels/:serverId')
export class ServerChannelController {
	@web.use()
	async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
		if (!req.user.servers.some(id => id === req.params.serverId)) {
			throw new HTTPError('UNKNOWN_SERVER')
		}
		next()
	}

	@web.get('/')
	async fetchChannels(req: Request, res: Response): Promise<void> {
		const channels = await Channel.find({
			serverId: req.params.serverId
		})

		res.json(channels)
	}

	@web.get('/:channelId')
	async fetchChannel(req: Request, res: Response): Promise<void> {
		const channel = await Channel.findOne({
			_id: req.params.channelId,
			serverId: req.params.serverId
		})

		if (!channel) {
			throw new HTTPError('UNKNOWN_CHANNEL')
		}

		res.json(channel)
	}

	@web.post('/')
	async createChannel(req: Request, res: Response): Promise<void> {
		req.check(CreateTextChannelSchema)

		const permissions = await Permissions.fetch(req.user, req.params.serverId)

		if (!permissions.has(Permissions.FLAGS.MANAGE_CHANNELS)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		const channel = await Channel.from({
			...req.body,
			serverId: req.params.serverId,
			type: ChannelTypes.TEXT
		}).save()

		res.json(channel)
	}

	@web.route('delete', '/:channelId')
	async deleteChannel(req: Request, res: Response): Promise<void> {
		const channel = await Channel.findOne({
			_id: req.params.channelId,
			serverId: req.params.serverId
		})

		if (!channel) {
			throw new HTTPError('UNKNOWN_CHANNEL')
		}

		const permissions = await Permissions.fetch(req.user, req.params.serverId)

		if (!permissions.has(Permissions.FLAGS.MANAGE_CHANNELS)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		await channel.delete()

		res.ok()
	}
}