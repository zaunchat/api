import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { HTTPError } from '../../errors'
import { LogoutUserSchema } from '../../structures'


@web.basePath('/auth/sessions')
export class SessionController {
	@web.get('/')
	async fetchMany(req: Request, res: Response): Promise<void> {
		const sessions = await req.user.sessions.loadItems()
		res.json(sessions)
	}

	@web.get('/:session_id')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const session = req.params.session_id


		if (!session) {
			throw new HTTPError('UNKNOWN_SESSION')
		}

		res.json(session)
	}


	@web.post('/logout/:session_id')
	async logout(req: Request, res: Response): Promise<void> {
		req.check(LogoutUserSchema)

		const session = req.user.sessions.getItems().find(s => {
			return s.token === req.body.token
		})

		if (!session) {
			throw new HTTPError('UNKNOWN_SESSION')
		}

		req.user.sessions.remove(session)

		await req.user.save()

		res.sendStatus(202)
	}
}