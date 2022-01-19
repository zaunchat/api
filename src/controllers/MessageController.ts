import { Controller, Context, Check, Next, Permission } from './Controller'
import { Message, CreateMessageSchema, UpdateMessageSchema } from '../structures'
import { Permissions } from '../utils'
import { gt, lt, gte } from 'pg-query-config'

export class MessageController extends Controller('/channels/:channel_id/messages') {
  async 'USE /'(ctx: Context, next: Next) {
    const permissions = await Permissions.from(ctx.request)

    if (!permissions.has(Permissions.FLAGS.VIEW_CHANNEL)) {
      ctx.throw('MISSING_PERMISSIONS')
    }

    next()
  }

  @Check(CreateMessageSchema)
  @Permission.has('SEND_MESSAGES')
  async 'POST /'(ctx: Context): Promise<Message> {
    const message = Message.from({
      ...ctx.body,
      author_id: ctx.user.id,
      channel_id: ctx.params.channel_id
    })

    if (message.isEmpty()) {
      ctx.throw('EMPTY_MESSAGE')
    }

    await message.save()

    return message
  }

  @Check(UpdateMessageSchema)
  async 'PATCH /:message_id'(ctx: Context): Promise<Message> {
    const { message_id, channel_id } = ctx.params

    const message = await Message.findOne({
      id: message_id,
      channel_id
    })

    if (message.author_id !== ctx.user.id) {
      ctx.throw('CANNOT_EDIT_MESSAGE_BY_OTHER')
    }

    ctx.body.edited_at = Date.now()

    await message.update(ctx.body)

    return message
  }

  async 'DELETE /:message_id'(ctx: Context): Promise<void> {
    const { message_id, channel_id } = ctx.params

    const message = await Message.findOne({
      id: message_id,
      channel_id
    })

    const permissions = await Permissions.from(ctx.request)

    if (message.author_id !== ctx.user.id && !permissions.has(Permissions.FLAGS.MANAGE_MESSAGES)) {
      ctx.throw('MISSING_PERMISSIONS')
    }

    await message.delete()
  }

  @Permission.has('READ_MESSAGE_HISTORY')
  @Check({
    limit: 'number|min:1|max:100|convert',
    before: 'snowflake|optional',
    after: 'snowflake|optional',
    around: 'snowflake|optional',
  }, 'query')
  async 'GET /'(ctx: Context): Promise<Message[]> {
    const {
      before,
      after,
      around,
      limit
    } = ctx.query as Record<string, string>

    const messages = await Message.find((sql) => {
      sql.where({ channel_id: ctx.params.channel_id })

      if (around) {
        sql.where({ id: gte(around) })
      } else {
        if (after) sql.where({ id: gt(after) })
        if (before) sql.where({ id: lt(before) })
      }
    }, Number(limit))

    return messages
  }

  @Permission.has('READ_MESSAGE_HISTORY')
  'GET /search'(_ctx: Context) {
    // Not Implemented yet.
    return 501
  }

  @Permission.has('READ_MESSAGE_HISTORY')
  'GET /:message_id'(ctx: Context): Promise<Message> {
    return Message.findOne({ id: ctx.params.message_id, channel_id: ctx.params.channel_id })
  }
}
