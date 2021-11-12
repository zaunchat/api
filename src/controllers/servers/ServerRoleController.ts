import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { Role, CreateRoleSchema } from '../../structures'
import { HTTPError } from '../../errors'
import { Permissions } from '../../utils'
import config from '../../config'


@web.basePath('/servers/:server_id/roles')
export class ServerRoleController {
	@web.use()
	async authentication(req: Request, _res: Response, next: NextFunction): Promise<void> {
		const server = req.user.servers.getItems().find((s) => {
			return s.id === req.params.server_id
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
		const roles = await Role.find({
			server: {
				id: req.params.server_id
			}
		})
		res.json(roles)
	}

	@web.get('/:role_id')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const role = await Role.findOne({
			id: req.params.role_id,
			server: {
				id: req.params.server_id
			}
		})

		if (!role) {
			throw new HTTPError('UNKNOWN_ROLE')
		}

		res.json(role)
	}

	@web.post('/')
	async create(req: Request, res: Response): Promise<void> {
		req.check(CreateRoleSchema)

		const server = req.server

		if (server.roles.length >= config.limits.server.roles) {
			throw new HTTPError('MAXIMUM_ROLES')
		}

		const permissions = await Permissions.fetch(req.user, server)

		if (!permissions.has(Permissions.FLAGS.MANAGE_ROLES)) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		const role = await Role.from({
			name: 'new role',
			server
		}).save()

		res.json(role)
	}
}