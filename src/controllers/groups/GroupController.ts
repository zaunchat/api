import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { CreateGroupSchema, Group, ChannelTypes } from '../../structures'
import { HTTPError } from '../../errors'
import { BASE_GROUP_PATH } from '.'
import { getaway } from '../../server'
import config from '../../../config'


@web.basePath(BASE_GROUP_PATH)
export class GroupController {
	@web.get('/')
	async fetchGroups(req: Request, res: Response): Promise<void> {
		const groups = await Group.find({
			recipients: req.user._id,
			deleted: false
		})
		res.json(groups)
	}

	@web.get('/:groupId')
	async fetchGroup(req: Request, res: Response): Promise<void> {
		const group = await Group.findOne({
			_id: req.params.groupId,
			recipients: req.user._id,
			deleted: false
		})

		if (!group) {
			throw new HTTPError('UNKNOWN_GROUP')
		}

		res.json(group)
	}

	@web.post('/')
	async createGroup(req: Request, res: Response): Promise<void> {
		req.check(CreateGroupSchema)

		const groupCount = await Group.count({
			recipients: req.user._id,
			deleted: false
		})

		if (groupCount >= config.limits.user.groups) {
			throw new HTTPError('MAXIMUM_GROUPS')
		}

		const group = await Group.from({
			name: req.body.name,
			ownerId: req.user._id,
			recipients: [req.user._id]
		}).save()

		await Promise.all(group.recipients.map((userId) => getaway.subscribe(userId, group._id)))

		getaway.publish(group._id, 'CHANNEL_CREATE', group)

		res.json(group)
	}

	@web.route('delete', '/:groupId')
	async deleteChannel(req: Request, res: Response): Promise<void> {
		const group = await Group.findOne({
			_id: req.params.groupId,
			recipients: req.user._id,
			deleted: false
		})

		if (!group) {
			throw new HTTPError('UNKNOWN_GROUP')
		}

		if (group.type === ChannelTypes.GROUP && group.ownerId !== req.user._id) {
			throw new HTTPError('MISSING_ACCESS')
		}

		await group.save({ deleted: true })

		getaway.publish(group._id, 'CHANNEL_DELETE', { _id: group._id })

		res.ok()
	}
}