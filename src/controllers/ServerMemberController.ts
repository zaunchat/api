import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { Member } from '../structures'
import { HTTPError } from '../errors'


@web.basePath('/servers/:serverId/members')
export class ServerMemberController {
	@web.use()
	async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
        if (!req.user.servers.some(id => id === req.params.serverId)) {
            throw new HTTPError('UNKNOWN_SERVER')
        }
		next()
	}

	@web.get('/')
	async fetchMembers(req: Request, res: Response): Promise<void> {
		const limit = 1000
		const members = await Member.find({ serverId: req.params.serverId }, { limit })
		res.json(members)
	}
}