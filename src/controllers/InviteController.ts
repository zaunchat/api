import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Invite, Member } from '../structures'
import { HTTPError } from '../errors'

@web.basePath('/invites')
export class InviteController {
	@web.get('/:invite_code')
	async fetch(req: Request, res: Response): Promise<void> {
		const invite = await Invite.findOne(`code = ${req.params.invite_code}`)
		res.json(invite)
	}


	@web.post('/:invite_code')
	async join(req: Request, res: Response): Promise<void> {
		const invite = await Invite.findOne(`code = ${req.params.invite_code}`)
		const alreadyJoined = await Member.findOne(`id = ${req.user.id} AND server_id = ${invite.server_id}`).catch(() => null)

		if (alreadyJoined) {
			throw new HTTPError('MISSING_ACCESS')
		}

		await Member.from({
			id: req.user.id,
			server_id: invite.server_id
		}).save()

		res.json({})
	}
}