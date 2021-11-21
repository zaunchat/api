import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { HTTPError } from '../../errors'
import { LogoutUserSchema, Session } from '../../structures'


@web.basePath('/auth/sessions')
export class SessionController {
	@web.get('/')
	async fetchMany(req: Request, res: Response): Promise<void> {
		const sessions = await Session.find(`user_id = ${req.user.id}`, ['id', 'info'])
		res.json(sessions)
	}

	@web.get('/:session_id')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const session = await Session.findOne(`id = ${req.params.session_id} AND user_id = ${req.user.id}`, ['info'])
		res.json(session)
	}


	@web.post('/logout/:session_id')
	async logout(req: Request, res: Response): Promise<void> {
		req.check(LogoutUserSchema)

		const session = await Session.findOne(`token = ${req.body.token} AND id = ${req.params.session_id}`)

		if (!session) {
			throw new HTTPError('UNKNOWN_SESSION')
		}

		await session.delete()

		res.sendStatus(202)
	}
}