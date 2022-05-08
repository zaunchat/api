import { Controller, Context, Limit, Next } from './Controller'
import { Channel, ChannelTypes, User, RelationshipStatus, DMChannel, PublicUser } from '../structures'
import { array } from 'pg-query-config'

@Limit('5/5s')
export class UserController extends Controller<Context> {
  'GET /:user_id'(ctx: Context): Promise<PublicUser> {
    return User.fetchPublicUser(ctx.params.user_id)
  }

  async 'GET /:user_id/dm'(ctx: Context): Promise<DMChannel> {
    const { user_id } = ctx.params
    const target = await User.fetchPublicUser(user_id)
    const exists = await Channel.findOne<DMChannel>({
      type: ChannelTypes.DM,
      recipients: array.lc([user_id])
    }).catch(() => null)

    if (exists) {
      return exists
    }

    const channel = Channel.from({
      type: ChannelTypes.DM,
      recipients: [ctx.user.id, target.id]
    })

    await channel.save()

    return channel
  }

  'GET /@me/relationships'(ctx: Context): Promise<PublicUser[]> {
    return ctx.user.fetchRelations()
  }

  'USE /@me/relationships/:target_id'(ctx: Context, next: Next) {
    if (ctx.params.target_id === ctx.user.id) ctx.throw('MISSING_ACCESS')
    next()
  }


  // Send/Request friend request
  async 'POST /@me/relationships/:target_id'(ctx: Context) {
    const relations = ctx.user.relations, targetId = ctx.params.target_id

    if (relations[targetId] === RelationshipStatus.BLOCKED_BY_OTHER) ctx.throw('BLOCKED_BY_OTHER')
    if (relations[targetId] === RelationshipStatus.FRIEND) ctx.throw('ALREADY_FRIENDS')
    if (relations[targetId] === RelationshipStatus.OUTGOING) ctx.throw('ALREADY_SENT_REQUEST')

    const target = await User.findOne({ id: targetId }), targetRelations = target.relations

    if (targetRelations[ctx.user.id] === RelationshipStatus.OUTGOING) { // Accept friend request.
      targetRelations[ctx.user.id] = relations[target.id] = RelationshipStatus.FRIEND
    } else { // Send friend request...
      relations[target.id] = RelationshipStatus.OUTGOING
      targetRelations[ctx.user.id] = RelationshipStatus.IN_COMING
    }

    await Promise.all([
      ctx.user.update({ relations }),
      target.update({ relations: targetRelations })
    ])
  }


  // Block people
  async 'PUT /@me/relationships/:target_id'(ctx: Context) {
    const relations = ctx.user.relations, targetId = ctx.params.target_id

    if (relations[targetId] === RelationshipStatus.BLOCKED) ctx.throw('BLOCKED')

    const target = await User.findOne({ id: targetId }), targetRelations = target.relations

    relations[target.id] = RelationshipStatus.BLOCKED

    if (targetRelations[ctx.user.id] !== RelationshipStatus.BLOCKED) {
      targetRelations[ctx.user.id] = RelationshipStatus.BLOCKED_BY_OTHER
    }
  }

  // Un(friend/block) people
  async 'DELETE /@me/relationships/:target_id'(ctx: Context) {
    const relations = ctx.user.relations, targetId = ctx.params.target_id

    if (!relations[targetId]) return 404

    const target = await User.findOne({ id: targetId }), targetRelations = target.relations

    delete relations[target.id]

    await ctx.user.update({ relations })

    if (targetRelations[ctx.user.id] === RelationshipStatus.FRIEND) {
      delete targetRelations[ctx.user.id]
      await target.update({ relations: targetRelations })
    }
  }
}
