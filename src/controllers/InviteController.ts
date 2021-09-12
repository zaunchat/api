import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Invite, Member } from '../structures'
import { HTTPError } from '../errors'

@web.basePath('/invites')
export class InviteController {
	@web.get('/:invite_code')
	async fetch(req: Request, res: Response): Promise<void> {
		const invite = await Invite.findOne({
			code: req.params.invite_code
		})

		if (!invite) {
			throw new HTTPError('UNKNON_INVITE')
		}

		res.json(invite)
	}


	@web.post('/:invite_code')
	async join(req: Request, res: Response): Promise<void> {
		const invite = await Invite.findOne({
			code: req.params.invite_code
		})

		if (!invite) {
			throw new HTTPError('UNKNON_INVITE')
		}

		const server = invite.channel.server

		if (req.user.servers.contains(server)) {
			throw new HTTPError('MISSING_ACCESS')
		}

		await Member.from({
			_id: req.user._id,
			server
		}).save()


		res.json(server)
	}
}