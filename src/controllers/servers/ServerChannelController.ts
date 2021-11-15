import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../../errors'
import { Channel, CreateTextChannelSchema, ChannelTypes, Member } from '../../structures'
import { Permissions } from '../../utils'
import config from '../../config'


@web.basePath('/channels/:server_id')
export class ServerChannelController {
	@web.use()
	async authentication(req: Request, _res: Response, next: NextFunction): Promise<void> {
		const exists = await Member.findOne(`id = ${req.user.id} AND server_id = ${req.params.server_id}`).catch(() => null)

		if (!exists) {
			throw new HTTPError('UNKNOWN_SERVER')
		}

		next()
	}

	@web.get('/')
	async fetchMany(req: Request, res: Response): Promise<void> {
		const channels = await Channel.find(`server_id = ${req.params.server_id}`)
		res.json(channels)
	}

	@web.get('/:channel_id')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const { server_id, channel_id } = req.params
		const channel = await Channel.findOne(`id = ${channel_id} AND server_id = ${server_id}`)
		res.json(channel)
	}

	@web.post('/')
	async create(req: Request, res: Response): Promise<void> {
		req.check(CreateTextChannelSchema)

		const channelCount = await Channel.count(`server_id = ${req.params.server_id}`)

		if (channelCount >= config.limits.server.channels) {
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
		const { server_id, channel_id } = req.params
		const channel = await Channel.findOne(`id = ${channel_id} AND server_id = ${server_id}`)
		const permissions = await Permissions.fetch(req.user, req.server)

		if (!permissions.has(Permissions.FLAGS.MANAGE_CHANNELS)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		await channel!.delete()

		res.sendStatus(202)
	}
}