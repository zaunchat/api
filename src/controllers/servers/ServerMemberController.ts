import { Controller, Context, Check, Next } from '../Controller'
import { Member, UpdateMemberSchema, Role } from '../../structures'
import { is, Permissions } from '../../utils'

export class ServerMemberController extends Controller('/servers/:server_id/members') {
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


  @Check({ limit: 'number|convert|min:1|max:1000|default:500' }, 'query')
  'GET /'(ctx: Context): Promise<Member[]> {
    return Member.find({ server_id: ctx.params.server_id }, Number(ctx.query.limit))
  }

  'GET /:member_id'(ctx: Context): Promise<Member> {
    return Member.findOne({ id: ctx.params.member_id, server_id: ctx.params.server_id })
  }


  @Check(UpdateMemberSchema)
  async 'PATCH /:member_id'(ctx: Context): Promise<Member> {
    const { server_id, member_id } = ctx.params
    
    const member = await Member.findOne({ id: member_id, server_id })
    const permissions = await Permissions.from(ctx.request)
    const changes: Record<string, unknown> = {}

    if ('nickname' in ctx.body) {
      if (ctx.user.id === member.id) {
        if (!permissions.has(Permissions.FLAGS.CHANGE_NICKNAME)) ctx.throw('MISSING_PERMISSIONS')
      } else {
        if (!permissions.has(Permissions.FLAGS.MANAGE_NICKNAMES)) ctx.throw('MISSING_PERMISSIONS')
      }
      changes.nickname = ctx.body.nickname
    }

    if (!is.nil(ctx.body.roles)) {
      if (!permissions.has(Permissions.FLAGS.MANAGE_ROLES)) ctx.throw('MISSING_PERMISSIONS')

      const roles = await Role.find({ server_id: ctx.params.server_id })

      changes.roles = []

      for (const roleId of ctx.body.roles) {
        const role = roles.find(r => r.id === roleId)

        if (!role) ctx.throw('UNKNOWN_ROLE');

        (<string[]>changes.roles).push(role!.id)
      }
    }

    await member.update(changes)

    return member
  }

  async 'DELETE /:member_id'(ctx: Context) {
    const { server_id, member_id } = ctx.params

    if (member_id !== ctx.user.id) {
      const permissions = await Permissions.from(ctx.request)
      if (!permissions.has(Permissions.FLAGS.KICK_MEMBERS)) ctx.throw('MISSING_PERMISSIONS')
    }

    const member = await Member.findOne({
      id: member_id,
      server_id
    })

    await member.delete()
  }
}
