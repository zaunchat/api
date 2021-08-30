import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Group } from '../structures'
import { HTTPError } from '../errors'
import { getaway } from '../server'
import { validator } from '../utils'
import config from '../../config'


@web.basePath('/groups')
export class GroupController {
    checks = {
        createGroup: validator.compile({
            name: { type: 'string' }
        })
    }

    @web.get('/')
    async fetchGroups(req: Request, res: Response): Promise<void> {
        const groups = await Group.find({
            deleted: false,
            recipients: req.user._id
        })
        res.json(groups)
    }

    @web.post('/')
    async createGroup(req: Request, res: Response): Promise<void> {
        const valid = this.checks.createGroup(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const groupCount = await Group.count({
            deleted: false,
            recipients: req.user._id
        })

        if (groupCount >= config.max.user.groups) {
            return void res.status(403).send(new HTTPError('MAXIMUM_GROUPS'))
        }

        const group = Group.from({
            name: req.body.name,
            ownerId: req.user._id,
            recipients: [req.user._id]
        })

        await group.save()

        getaway.emit('CHANNEL_CREATE', group)

        res.json(group)
    }

    @web.get('/:groupId')
    async fetchGroup(req: Request, res: Response): Promise<void> {
        const group = await Group.findOne({
            _id: req.params.groupId,
            deleted: false
        })

        if (!group) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        if (group.ownerId !== req.user._id && !group.recipients.some((id) => id === req.user._id)) {
            return void res.status(400).send(new HTTPError('MISSING_ACCESS'))
        }

        res.json(group)
    }


    @web.route('delete', '/:groupId')
    async deleteGroup(req: Request, res: Response): Promise<void> {
        const group = await Group.findOne({
            deleted: false,
            _id: req.params.groupId
        })

        if (!group) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        if (group.ownerId !== req.user._id) {
            return void res.status(400).send(new HTTPError('MISSING_ACCESS'))
        }

        await group.save({ deleted: true })

        getaway.emit('CHANNEL_DELETE', { _id: group._id })

        res.sendStatus(202)
    }
}