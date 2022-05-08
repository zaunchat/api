import { Context, Check, Limit } from '../Controller'
import { Channel, ChannelTypes, CreateGroupSchema, GroupChannel, User } from '../../structures'
import { Permissions } from '../../utils'
import { array } from 'pg-query-config'
import config from '../../config'
import { Controller } from '@itchatt/controllers'

@Limit('5/5s')
export class ChannelController extends Controller {
  'GET /'(ctx: Context) {
    return Channel.find({ recipients: array.lc([ctx.user.id]) })
  }

  'GET /:channel_id'(ctx: Context) {
    return Channel.findOne({
      id: ctx.params.channel_id,
      recipients: array.lc([ctx.user.id])
    })
  }

  @Check(CreateGroupSchema)
  async 'POST /'(ctx: Context) {
    const groupCount = await Channel.count(`type = ${ChannelTypes.GROUP} AND recipients::jsonb ? ${ctx.user.id}`)

    if (groupCount >= config.limits.user.groups) {
      ctx.throw('MAXIMUM_GROUPS')
    }

    const group = Channel.from({
      type: ChannelTypes.GROUP,
      name: ctx.body.name,
      owner_id: ctx.user.id,
      recipients: [ctx.user.id]
    })

    await group.save()

    return group
  }

  async 'POST /:group_id/:user_id'(ctx: Context) {
    const { user_id, group_id } = ctx.params

    const [group, target] = await Promise.all([
      Channel.findOne<GroupChannel>({
        id: group_id,
        type: ChannelTypes.GROUP,
        recipients: array.lc([ctx.user.id])
      }),
      User.findOne({ id: user_id })
    ])

    if (group.recipients.length >= config.limits.group.members) {
      ctx.throw('MAXIMUM_GROUP_MEMBERS')
    }

    if (group.recipients.includes(target.id)) {
      ctx.throw('MISSING_ACCESS')
    }

    await group.update({
      recipients: [...group.recipients, target.id]
    })

    return group
  }

  async 'DELETE /:group_id/:user_id'(ctx: Context) {
    const { user_id, group_id } = ctx.params

    const [group, target] = await Promise.all([
      Channel.findOne<GroupChannel>({
        id: group_id,
        type: ChannelTypes.GROUP,
        recipients: array.lc([ctx.user.id])
      }),
      User.findOne({ id: user_id })
    ])

    if (ctx.user.id === group.owner_id && ctx.user.id === target.id) {
      ctx.throw('MISSING_ACCESS')
    }

    if (!group.recipients.includes(target.id)) {
      ctx.throw('UNKNOWN_MEMBER')
    }

    const permissions = await Permissions.fetch({
      user: ctx.user,
      channel: group
    })

    if (!permissions.has('KICK_MEMBERS')) {
      ctx.throw('MISSING_PERMISSIONS')
    }

    await group.update({
      recipients: group.recipients.filter((id) => id !== target.id)
    })
  }

  async 'DELETE /:channel_id'(ctx: Context) {
    const channel = await Channel.findOne<GroupChannel>({
      id: ctx.params.channel_id,
      type: ChannelTypes.GROUP,
      recipients: array.lc([ctx.user.id])
    })

    if (channel.owner_id !== ctx.user.id) {
      ctx.throw('MISSING_ACCESS')
    }

    await channel.delete()
  }
}
