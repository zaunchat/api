import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../../errors'
import { Channel, CreateTextChannelSchema, ChannelTypes } from '../../structures'
import { Permissions } from '../../utils'
import config from '../../../config'


@web.basePath('/channels/:server_id')
export class ServerChannelController {
	@web.use()
	async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
		const server = req.user.servers.getItems().find((s) => {
			return s._id === req.params.server_id
		})

		if (!server) {
			throw new HTTPError('UNKNOWN_SERVER')
		}

		Object.defineProperty(req, 'server', {
			value: server
		})

		next()
	}

	@web.get('/')
	async fetchMany(req: Request, res: Response): Promise<void> {
		const channels = await Channel.find({
			server: {
				_id: req.params.server_id
			}
		})
		res.json(channels)
	}

	@web.get('/:channel_id')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const channel = await Channel.findOne({
			_id: req.params.channel_id,
			server: {
				_id: req.params.server_id
			}
		})

		if (!channel) {
			throw new HTTPError('UNKNOWN_CHANNEL')
		}

		res.json(channel)
	}

	@web.post('/')
	async create(req: Request, res: Response): Promise<void> {
		req.check(CreateTextChannelSchema)

		const server = req.server

		if (server.channels.length >= config.limits.server.channels) {
			throw new HTTPError('MAXIMUM_CHANNELS')
		}

		const permissions = await Permissions.fetch(req.user, server)

		if (!permissions.has(Permissions.FLAGS.MANAGE_CHANNELS)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		const channel = await Channel.from({
			...req.body,
			server: server,
			type: ChannelTypes.TEXT // TODO: Add category type
		}).save()

		res.json(channel)
	}

	@web.route('delete', '/:channel_id')
	async delete(req: Request, res: Response): Promise<void> {
		const channel = await Channel.findOne({
			_id: req.params.channel_id,
			server: {
				_id: req.params.server_id
			}
		})

		if (!channel) {
			throw new HTTPError('UNKNOWN_CHANNEL')
		}

		const permissions = await Permissions.fetch(req.user, req.server)

		if (!permissions.has(Permissions.FLAGS.MANAGE_CHANNELS)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		await channel.delete()

		res.ok()
	}
}