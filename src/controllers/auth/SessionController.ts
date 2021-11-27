import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { LogoutUserSchema, Session } from '../../structures'

@web.basePath('/auth/sessions')
export class SessionController {
  @web.get('/')
  async fetchMany(req: Request, res: Response): Promise<void> {
    const sessions = await Session.find(sql => sql.select(['id', 'info']).where({ user_id: req.user.id }))
    res.json(sessions)
  }

  @web.get('/:session_id')
  async fetchOne(req: Request, res: Response): Promise<void> {
    const session = await Session.findOne(sql => sql.select('info').where({
      id: req.params.session_id,
      user_id: req.user.id
    }))
    res.json(session)
  }


  @web.post('/logout/:session_id')
  async logout(req: Request, res: Response): Promise<void> {
    req.check(LogoutUserSchema)

    const session = await Session.findOne({
      id: req.params.session_id,
      token: req.body.token
    })

    await session.delete()

    res.sendStatus(202)
  }
}
