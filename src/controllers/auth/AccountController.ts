import { Controller, Context, Check, Limit, Captcha } from '../Controller'
import { User, Session, CreateUserSchema, LoginUserSchema } from '../../structures'
import { email } from '../../utils'
import config from '../../config'
import argon2 from 'argon2'

//@AntiProxy()
@Limit('30/1h --ip')
export class AccountController extends Controller {
  @Captcha()
  @Check(LoginUserSchema)
  async 'POST /login'(ctx: Context) {
    const user = await User.findOne({ email: ctx.body.email })

    if (!user.verified) {
      ctx.throw('USER_NOT_VERIFIED')
    }

    if (!await argon2.verify(ctx.body.password, user.password)) {
      ctx.throw('INVALID_PASSWORD')
    }

    const session = Session.from({
      user_id: user.id
    })

    await session.save()

    return {
      token: session.token,
      id: user.id
    }
  }

  @Captcha()
  @Check(CreateUserSchema)
  async 'POST /register'(ctx: Context) {
    const user = User.from({
      username: ctx.body.username,
      email: ctx.body.email,
      password: await argon2.hash(ctx.body.password),
      verified: !config.smtp.enabled
    })

    await user.save()

    // No email verification :(
    if (!config.smtp.enabled) {
      return 201 // Created
    }

    try {
      await email.send(user)
      return 201
    } catch (err) {
      await user.delete()
      throw err
    }
  }

  async 'GET /verify/:user_id/:code'(ctx: Context) {
    const { user_id, code } = ctx.params

    const verified = await email.verify(user_id, code)

    if (!verified) {
      ctx.throw('UNKNOWN_TOKEN')
    }

    const user = await User.findOne({ id: user_id })

    await user.update({ verified: true })

    ctx.response.redirect(config.endpoints.app)
  }
}
