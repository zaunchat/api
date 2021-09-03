import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { Group } from '../../structures'
import { HTTPError } from '../../errors'
import { Permissions } from '../../utils'
import { BASE_GROUP_PATH } from '.'
import config from '../../../config'


@web.basePath(`${BASE_GROUP_PATH}/:groupId/members`)
export class GroupMemberController {
	@web.use()
	async hasAccess(req: Request, _res: Response, next: NextFunction): Promise<void> {
		const group = await Group.findOne({
			_id: req.params.groupId,
			recipients: req.user._id
		})

		if (!group) {
			throw new HTTPError('UNKNOWN_CHANNEL')
		}

		Object.defineProperty(req, 'group', {
			value: group
		})

		next()
	}

	@web.post('/:userId')
	async addMember(req: Request, res: Response): Promise<void> {
		const group = (req as unknown as { group: Group }).group

		if (group.recipients.length >= config.limits.group.members) {
			throw new HTTPError('MAXIMUM_GROUP_MEMBERS')
		}

		if (group.recipients.includes(req.params.userId as Snowflake)) {
			throw new HTTPError('MISSING_ACCESS')
		}

		group.recipients.push(req.params.userId as Snowflake)

		await group.save()

		res.json(group)
	}

	@web.route('delete', '/:userId')
	async removeMember(req: Request, res: Response): Promise<void> {
		const group = (req as unknown as { group: Group }).group

		if (req.user._id === group.ownerId && req.user._id === req.params.userId) {
			throw new HTTPError('MISSING_ACCESS')
		}

		if (!group.recipients.includes(req.params.userId as Snowflake)) {
			throw new HTTPError('UNKNOWN_MEMBER')
		}

		const permissions = await Permissions.fetch(req.user, null, group)

		if (!permissions.has('KICK_MEMBERS')) {
			throw new HTTPError('MISSING_PERMISSIONS')
		}

		await group.save({
			recipients: group.recipients.filter((id) => id !== req.params.userId)
		})

		res.ok()
	}
}