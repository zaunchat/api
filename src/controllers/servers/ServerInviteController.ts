import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../../errors'
import { Channel, Invite, Member } from '../../structures'
import { Permissions } from '../../utils'

@web.basePath('/servers/:server_id/invites')
export class ServerInviteController {
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
		const limit = 100 // TODO: Add Limit option

		const invites = await Invite.find(`server_id = ${req.params.server_id}`, undefined, limit)

		res.json(invites)
	}

	@web.get('/:invite_code')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const { invite_code, server_id } = req.params
		const invite = await Invite.find(`code = ${invite_code} AND server_id = ${server_id}`)
		res.json(invite)
	}

	@web.post('/:channel_id')
	async create(req: Request, res: Response): Promise<void> {
		const channel = await Channel.findOne(`id = ${req.params.channel_id}`)
		const permissions = await Permissions.fetch({
			user: req.user,
			server: req.params.server_id as ID,
			channel
		})

		if (!permissions.has(Permissions.FLAGS.INVITE_OTHERS)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		const invite = Invite.from({
			inviter_id: req.user.id,
			channel_id: channel.id,
			server_id: req.params.server_id as ID
		})

		await invite.save()

		res.json({ code: invite.code })
	}
}