import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { Member, CreateMemberSchema, Server, User } from '../../structures'
import { HTTPError } from '../../errors'
import { Permissions } from '../../utils'


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

	@web.get('/:memberId')
	async fetchMember(req: Request, res: Response): Promise<void> {
		const member = await Member.findOne({
			_id: req.params.memberId,
			serverId: req.params.serverId
		})

		if (!member) {
			throw new HTTPError('UNKNOWN_MEMBER')
		}

		res.json(member)
	}

	@web.patch('/:memberId')
	async editMember(req: Request, res: Response): Promise<void> {
		req.check(CreateMemberSchema)

		const { serverId, memberId } = req.params as Record<string, Snowflake>
		const member = await Member.findOne({
			_id: memberId,
			serverId: serverId
		})

		if (!member) {
			throw new HTTPError('UNKNOWN_MEMBER')
		}

		const server = await Server.findOne({ _id: serverId }) as Server
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
			for (const roleId of req.body.roles) {
				if (!server.roles.includes(roleId)) throw new HTTPError('UNKNOWN_ROLE')
				member.roles.push(roleId)
			}
		}

		await member.save()

		res.json(member)
	}

	@web.route('delete', '/:memberId')
	async kickMember(req: Request, res: Response): Promise<void> {
		const { serverId, memberId } = req.params as Record<string, Snowflake>

		if (memberId !== req.user._id) {
			const permissions = await Permissions.fetch(req.user, serverId)
			if (!permissions.has(Permissions.FLAGS.KICK_MEMBERS)) {
				throw new HTTPError('MISSING_PERMISSIONS')
			}
		}

		const [member, user] = await Promise.all([
			Member.findOne({
				_id: memberId,
				serverId: serverId
			}),
			User.findOne({
				_id: memberId
			})
		])

		if (!member) {
			throw new HTTPError('UNKNOWN_MEMBER')
		}

		if (!user) {
			throw new HTTPError('UNKNOWN_USER')
		}

		await member.delete()

		res.ok()
	}
}