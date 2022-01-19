import { Controller, Context, Check, Limit } from '../Controller'
import { LogoutUserSchema, Session } from '../../structures'

@Limit('30/1h')
export class SessionController extends Controller('/auth/sessions') {
  'GET /'(ctx: Context): Promise<Session[]> {
    return Session.find(sql => sql.select(['id', 'info']).where({ user_id: ctx.user.id }))
  }

  'GET /:session_id'(ctx: Context): Promise<Session> {
    return Session.findOne(sql => sql.select('info').where({
      id: ctx.params.session_id,
      user_id: ctx.user.id
    }))
  }

  @Check(LogoutUserSchema)
  async 'POST /logout/:session_id'(ctx: Context) {
    const session = await Session.findOne({
      id: ctx.params.session_id,
      token: ctx.body.token
    })

    await session.delete()
  }
}
