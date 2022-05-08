import { Controller, Context, Check, Next, Permission } from '../Controller'
import { Role, CreateRoleSchema, Member } from '../../structures'
import config from '../../config'


export class ServerRoleController extends Controller {
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

  'GET /'(ctx: Context): Promise<Role[]> {
    return Role.find({ server_id: ctx.params.server_id })
  }

  'GET /:role_id'({ params: { server_id, role_id } }: Context) {
    return Role.findOne({ id: role_id, server_id })
  }

  @Check(CreateRoleSchema)
  @Permission.has('MANAGE_ROLES')
  async 'POST /'(ctx: Context): Promise<Role> {
    const roleCount = await Role.count(`server_id = ${ctx.params.server_id}`)

    if (roleCount >= config.limits.server.roles) {
      ctx.throw('MAXIMUM_ROLES')
    }

    const role = Role.from({
      ...ctx.body,
      server_id: ctx.params.server_id
    })

    await role.save()

    return role
  }
}
