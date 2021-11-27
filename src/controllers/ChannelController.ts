import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, CreateGroupSchema, User } from '../structures'
import { Permissions } from '../utils'
import config from '../config'
import { array } from 'pg-query-config'


@web.basePath('/channels/@me')
export class ChannelController {
  @web.get('/')
  async fetchMany(req: Request, res: Response): Promise<void> {
    const channels = await Channel.find({ recipients: array.lc([req.user.id]) })
    res.json(channels)
  }

  @web.get('/:channel_id')
  async fetchOne(req: Request, res: Response): Promise<void> {
    const channel = await Channel.findOne({
      id: req.params.channel_id,
      recipients: array.lc([req.user.id])
    })
    res.json(channel)
  }

  @web.post('/')
  async create(req: Request, res: Response): Promise<void> {
    req.check(CreateGroupSchema)

    const groupCount = await Channel.count(`type = ${ChannelTypes.GROUP} AND recipients::jsonb ? ${req.user.id}`)

    if (groupCount >= config.limits.user.groups) {
      req.throw('MAXIMUM_GROUPS')
    }

    const group = Channel.from({
      type: ChannelTypes.GROUP,
      name: req.body.name,
      owner_id: req.user.id,
      recipients: [req.user.id]
    })

    await group.save()

    res.json(group)
  }

  @web.post('/:group_id/:user_id')
  async add(req: Request, res: Response): Promise<void> {
    const { user_id, group_id } = req.params

    const [group, target] = await Promise.all([
      Channel.findOne({
        id: group_id,
        type: ChannelTypes.GROUP,
        recipients: array.lc([req.user.id])
      }),
      User.findOne({ id: user_id })
    ])

    if (group.recipients.length >= config.limits.group.members) {
      req.throw('MAXIMUM_GROUP_MEMBERS')
    }

    if (group.recipients.includes(target.id)) {
      req.throw('MISSING_ACCESS')
    }

    await group.update({
      recipients: [...group.recipients, target.id]
    })

    res.json(group)
  }

  @web.route('delete', '/:group_id/:user_id')
  async kick(req: Request, res: Response): Promise<void> {
    const { user_id, group_id } = req.params

    const [group, target] = await Promise.all([
      Channel.findOne({
        id: group_id,
        type: ChannelTypes.GROUP,
        recipients: array.lc([req.user.id])
      }),
      User.findOne({ id: user_id })
    ])

    if (req.user.id === group.owner_id && req.user.id === target.id) {
      req.throw('MISSING_ACCESS')
    }

    if (!group.recipients.includes(target.id)) {
      req.throw('UNKNOWN_MEMBER')
    }

    const permissions = await Permissions.fetch({
      user: req.user,
      channel: group
    })

    if (!permissions.has('KICK_MEMBERS')) {
      req.throw('MISSING_PERMISSIONS')
    }

    await group.update({
      recipients: group.recipients.filter((id) => id !== target.id)
    })

    res.sendStatus(202)
  }

  @web.route('delete', '/:channel_id')
  async delete(req: Request, res: Response): Promise<void> {
    const channel = await Channel.findOne({
      id: req.params.channel_id,
      type: ChannelTypes.GROUP,
      recipients: array.lc([req.user.id])
    })

    if (channel.type === ChannelTypes.GROUP && channel.owner_id !== req.user.id) {
      req.throw('MISSING_ACCESS')
    }

    await channel.delete()

    res.sendStatus(202)
  }
}
