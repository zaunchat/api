import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { Member, CreateMemberSchema } from '../../structures'
import { HTTPError } from '../../errors'
import { Permissions } from '../../utils'


@web.basePath('/servers/:server_id/members')
export class ServerMemberController {
	@web.use()
	async authentication(req: Request, _res: Response, next: NextFunction): Promise<void> {
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
		const limit = 1000 // TODO: Add Limit option

		const members = await Member.find({
			server: {
				_id: req.params.server_id
			}
		}, { limit })

		res.json(members)
	}

	@web.get('/:member_id')
	async fetchOne(req: Request, res: Response): Promise<void> {
		const member = await Member.findOne({
			_id: req.params.member_id,
			server: {
				_id: req.params.server_id
			}
		})

		if (!member) {
			throw new HTTPError('UNKNOWN_MEMBER')
		}

		res.json(member)
	}

	@web.patch('/:member_id')
	async edit(req: Request, res: Response): Promise<void> {
		req.check(CreateMemberSchema)

		const { server_id, member_id } = req.params as Record<string, ID>
		const member = await Member.findOne({
			_id: member_id,
			server: {
				_id: server_id
			}
		})

		if (!member) {
			throw new HTTPError('UNKNOWN_MEMBER')
		}

		const server = req.server
		const permissions = await Permissions.fetch(req.user, server)


		if ('nickname' in req.body) {
			if (req.user._id === member._id) {
				if (!permissions.has(Permissions.FLAGS.CHANGE_NICKNAME)) throw new HTTPError('MISSING_PERMISSIONS')
			} else {
				if (!permissions.has(Permissions.FLAGS.MANAGE_NICKNAMES)) throw new HTTPError('MISSING_PERMISSIONS')
			}
			member.nickname = req.body.nickname ? req.body.nickname : void 0
		}

		if (req.body.roles) {
			if (!permissions.has(Permissions.FLAGS.MANAGE_ROLES)) throw new HTTPError('MISSING_PERMISSIONS')

			const roles = server.roles.getItems()

			member.roles.removeAll()

			for (const role_id of req.body.roles) {
				const role = roles.find(r => r._id === role_id)
				if (!role) throw new HTTPError('UNKNOWN_ROLE')
				member.roles.add(role)
			}
		}

		await member.save()

		res.json(member)
	}

	@web.route('delete', '/:member_id')
	async kick(req: Request, res: Response): Promise<void> {
		const { server_id, member_id } = req.params as Record<string, ID>

		if (member_id !== req.user._id) {
			const permissions = await Permissions.fetch(req.user, server_id)
			if (!permissions.has(Permissions.FLAGS.KICK_MEMBERS)) {
				throw new HTTPError('MISSING_PERMISSIONS')
			}
		}

		const member = await Member.findOne({
			_id: member_id,
			server: {
				_id: server_id
			}
		})

		if (!member) {
			throw new HTTPError('UNKNOWN_MEMBER')
		}

		await member.delete()

		res.sendStatus(202)
	}
}