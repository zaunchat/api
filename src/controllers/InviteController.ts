import { Controller, Context } from './Controller'
import { Invite, Member } from '../structures'

export class InviteController extends Controller('/invites') {
  'GET /:invite_code'(ctx: Context): Promise<Invite> {
    return Invite.findOne({ code: ctx.params.invite_code })
  }

  async 'POST /:invite_code'(ctx: Context) {
    const invite = await Invite.findOne({ code: ctx.params.invite_code })
    const alreadyJoined = await Member.findOne({
      id: ctx.user.id,
      server_id: invite.server_id
    }).catch(() => null)

    if (alreadyJoined) {
      ctx.throw('MISSING_ACCESS')
    }

    await Promise.all([
      invite.update({ uses: invite.uses + 1 }),
      Member.from({
        id: ctx.user.id,
        server_id: invite.server_id
      }).save()
    ])
  }
}
