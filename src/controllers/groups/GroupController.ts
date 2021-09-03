import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Group, CreateGroupSchema } from '../../structures'
import { HTTPError } from '../../errors'
import { BASE_GROUP_PATH } from '.'
import config from '../../../config'


@web.basePath(BASE_GROUP_PATH)
export class GroupController {
	@web.get('/')
	async fetchGroups(req: Request, res: Response): Promise<void> {
		const groups = await Group.find({
			recipients: req.user._id	
		})
		res.json(groups)
	}

	@web.get('/:groupId')
	async fetchGroup(req: Request, res: Response): Promise<void> {
		const group = await Group.findOne({
			_id: req.params.groupId,
			recipients: req.user._id
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
			recipients: req.user._id
		})

		if (groupCount >= config.limits.user.groups) {
			throw new HTTPError('MAXIMUM_GROUPS')
		}

		const group = await Group.from({
			name: req.body.name,
			ownerId: req.user._id,
			recipients: [req.user._id]
		}).save()

		res.json(group)
	}

	@web.route('delete', '/:groupId')
	async deleteGroup(req: Request, res: Response): Promise<void> {
		const group = await Group.findOne({
			_id: req.params.groupId,
			recipients: req.user._id
		})

		if (!group) {
			throw new HTTPError('UNKNOWN_GROUP')
		}

		if (group.ownerId !== req.user._id) {
			throw new HTTPError('MISSING_ACCESS')
		}

		await group.delete()

		res.ok()
	}
}