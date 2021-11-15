import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { Role, CreateRoleSchema, Member } from '../../structures'
import { HTTPError } from '../../errors'
import { Permissions } from '../../utils'
import config from '../../config'


@web.basePath('/servers/:server_id/roles')
export class ServerRoleController {
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
		const roles = await Role.find(`server_id = ${req.params.server_id}`)
		res.json(roles)
	}

	@web.get('/:role_id')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const { server_id, role_id } = req.params
		const role = await Role.findOne(`id = ${role_id} AND server_id = ${server_id}`)
		res.json(role)
	}

	@web.post('/')
	async create(req: Request, res: Response): Promise<void> {
		req.check(CreateRoleSchema)

		const roleCount = await Role.count(`server_id = ${req.params.server_id}`)

		if (roleCount >= config.limits.server.roles) {
			throw new HTTPError('MAXIMUM_ROLES')
		}

		const permissions = await Permissions.fetch(req.user, server)

		if (!permissions.has(Permissions.FLAGS.MANAGE_ROLES)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		const role = await Role.from({
			name: 'new role',
			server_id: req.params.server_id as ID
		}).save()

		res.json(role)
	}
}