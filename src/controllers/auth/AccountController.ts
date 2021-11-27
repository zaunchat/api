import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { User, Session, CreateUserSchema, LoginUserSchema } from '../../structures'
import { is, email } from '../../utils'
import config from '../../config'
import argon2 from 'argon2'


@web.basePath('/auth/accounts')
export class AccountController {
  @web.post('/login')
  async login(req: Request, res: Response): Promise<void> {
    req.check(LoginUserSchema)

    const { email, password } = req.body

    if (!is.email(email)) {
      req.throw('INVALID_EMAIL')
    }

    const user = await User.findOne({ email })

    if (!user.verified) {
      req.throw('USER_NOT_VERIFIED')
    }

    if (!await argon2.verify(password, user.password)) {
      req.throw('INVALID_PASSWORD')
    }

    const session = Session.from({
      user_id: user.id
    })

    await session.save()

    res.json({
      token: session.token,
      id: user.id
    })
  }

  @web.post('/register')
  async register(req: Request, res: Response): Promise<void> {
    req.check(CreateUserSchema)

    const { username, password } = req.body as Record<string, string>

    if (!is.email(req.body.email)) {
      req.throw('INVALID_EMAIL')
    }


    const exists = await User.findOne(sql => sql
      .select(['username', 'email'])
      .where({ email: req.body.email })
      .orWhere([{ username }])).catch(() => null)


    if (exists) {
      if (username === exists.username) {
        req.throw('USERNAME_TAKEN')
      } else {
        req.throw('EMAIL_ALREADY_IN_USE')
      }
    }

    const user = User.from({
      username,
      email: req.body.email,
      password: await argon2.hash(password),
      verified: !config.smtp.enabled
    })

    await user.save()

    if (!config.smtp.enabled) {
      return void res.redirect(`${config.endpoints.main}/auth/login`)
    }

    try {
      await email.send(user)
      res.json({ message: 'Check your email' })
    } catch (err) {
      await user.delete()
      throw err
    }
  }

  @web.get('/verify/:user_id/:code')
  async verify(req: Request, res: Response): Promise<void> {
    const { user_id, code } = req.params as { user_id: ID; code: string }

    const verified = await email.verify(user_id, code)

    if (!verified) {
      req.throw('UNKNOWN_TOKEN')
    }

    const user = await User.findOne({ id: user_id })

    await user.update({ verified: true })

    res.redirect(`${config.endpoints.main}/auth/login`)
  }
}
