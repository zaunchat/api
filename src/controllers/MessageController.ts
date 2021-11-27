import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { CreateMessageSchema, Message } from '../structures'
import { is, Permissions } from '../utils'
import { QueryConfig as QueryBuilder, gt, lt, gte } from 'pg-query-config'
import config from '../config'

@web.basePath('/channels/:channel_id/messages')
export class MessageController {
  @web.use()
  async authentication(req: Request, _res: Response, next: NextFunction): Promise<void> {
    const permissions = await Permissions.fetch({
      user: req.user,
      channel: req.params.channel_id
    })

    if (!permissions.has(Permissions.FLAGS.VIEW_CHANNEL)) {
      req.throw('MISSING_PERMISSIONS')
    }

    Object.defineProperties(req, {
      permissions: {
        value: permissions
      }
    })

    next()
  }

  @web.post('/')
  async send(req: Request, res: Response): Promise<void> {
    if (!req.permissions.has(Permissions.FLAGS.SEND_MESSAGES)) {
      req.throw('MISSING_PERMISSIONS')
    }

    req.check(CreateMessageSchema)

    const message = Message.from({
      ...req.body,
      author_id: req.user.id,
      channel_id: req.params.channel_id
    })

    if (message.isEmpty()) {
      req.throw('EMPTY_MESSAGE')
    }

    if ((message.content?.length ?? 0) > config.limits.message.length) {
      req.throw('MAXIMUM_MESSAGE_LENGTH')
    }

    if (message.replies.length > config.limits.message.replies) {
      req.throw('TOO_MANY_REPLIES')
    }

    if (message.attachments.length > config.limits.message.attachments) {
      req.throw('TOO_MANY_ATTACHMENTS')
    }

    await message.save()

    res.json(message)
  }

  @web.get('/')
  async fetchMany(req: Request, res: Response): Promise<void> {
    if (!req.permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
      req.throw('MISSING_PERMISSIONS')
    }

    const {
      before,
      after,
      around
    } = req.query

    const limit = Number(req.query.limit ?? 50)

    if (isNaN(limit) || limit > 100 || limit < 0) {
      req.throw('MISSING_ACCESS')
    }

    const query = new QueryBuilder({ limit, table: Message.tableName })

    query.where({
      channel_id: req.params.channel_id
    })

    if (is.snowflake(around)) {
      query.where({ id: gte(around) })
    } else {
      if (is.snowflake(after)) query.where({ id: gt(after) })
      if (is.snowflake(before)) query.where({ id: lt(before) })
    }

    const messages = await Message.find(() => query)

    res.json(messages)
  }


  @web.get('/:message_id')
  async fetchOne(req: Request, res: Response): Promise<void> {
    if (!req.permissions.has(Permissions.FLAGS.READ_MESSAGE_HISTORY)) {
      req.throw('MISSING_PERMISSIONS')
    }

    const { message_id, channel_id } = req.params
    const message = await Message.findOne({
      id: message_id,
      channel_id
    })

    res.json(message)
  }

  @web.patch('/:message_id')
  async edit(req: Request, res: Response): Promise<void> {
    req.check(CreateMessageSchema)

    const { message_id, channel_id } = req.params

    const message = await Message.findOne({
      id: message_id,
      channel_id
    })

    if (message.author_id !== req.user.id) {
      req.throw('CANNOT_EDIT_MESSAGE_BY_OTHER')
    }

    await message.update(req.body)

    res.json(message)
  }

  @web.route('delete', '/:message_id')
  async delete(req: Request, res: Response): Promise<void> {
    const { message_id, channel_id } = req.params
    const message = await Message.findOne({
      id: message_id,
      channel_id
    })

    if (message.author_id !== req.user.id && !req.permissions.has(Permissions.FLAGS.MANAGE_MESSAGES)) {
      req.throw('MISSING_PERMISSIONS')
    }

    await message.delete()

    res.sendStatus(202)
  }
}
