import { Controller, Context, Check, Permission, Next } from '../Controller'
import { Channel, CreateServerChannelSchema, ChannelTypes, Member, ServerChannel } from '../../structures'
import config from '../../config'


export class ServerChannelController extends Controller {
  async 'USE /'(ctx: Context, next: Next) {
    const exists = await Member.findOne({
      id: ctx.user.id,
      server_id: ctx.params.server_id
    }).catch(() => null)

    if (!exists) {
      ctx.throw('UNKNOWN_SERVER')
    }

    next()
  }

  'GET /'(ctx: Context) {
    return Channel.find({ server_id: ctx.params.server_id })
  }

  'GET /:channel_id'({ params }: Context) {
    return Channel.findOne<ServerChannel>({
      id: params.channel_id,
      server_id: params.server_id
    })
  }


  @Check(CreateServerChannelSchema)
  @Permission.has('MANAGE_CHANNELS')
  async 'POST /'(ctx: Context) {
    const server_id = ctx.params.server_id
    const channelCount = await Channel.count(`server_id = ${server_id}`)

    if (channelCount >= config.limits.server.channels) {
      ctx.throw('MAXIMUM_CHANNELS')
    }

    let channel!: Channel

    switch (ctx.body.type as ChannelTypes) {
      case ChannelTypes.TEXT:
        channel = Channel.from({
          ...ctx.body,
          type: ChannelTypes.TEXT,
          server_id: server_id,
        })
        break
      case ChannelTypes.CATEGORY:
        channel = Channel.from({
          ...ctx.body,
          type: ChannelTypes.CATEGORY,
          server_id: server_id
        })
        break
      default:
        ctx.throw('INVALID_CHANNEL_TYPE')
    }

    await channel.save()

    return channel
  }


  @Permission.has('MANAGE_CHANNELS')
  async 'DELETE /:channel_id'(ctx: Context): Promise<void> {
    const channel = await Channel.findOne<ServerChannel>({
      id: ctx.params.channel_id,
      server_id: ctx.params.server_id
    })

    await channel.delete()
  }
}
