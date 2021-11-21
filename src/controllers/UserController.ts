import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, PUBLIC_USER_PROPS, User } from '../structures'


@web.basePath('/users')
export class UserController {
  @web.get('/:user_id')
  async fetchOne(req: Request, res: Response): Promise<void> {
    const user = await User.findOne(`id = ${req.params.user_id}`, PUBLIC_USER_PROPS)
    res.json(user)
  }

  @web.get('/@me/relationships')
  async fetchRelationships(req: Request, res: Response): Promise<void> {
    const relationships = await req.user.fetchRelations()
    res.json(relationships)
  }

  @web.get('/:user_id/dm')
  async openDM(req: Request, res: Response): Promise<void> {
    const { user_id } = req.params as Record<string, ID>
    const target = await User.findOne(`id = ${user_id}`)
    const exists = await Channel.findOne(`type = ${ChannelTypes.DM} AND recipients::jsonb ? ${user_id}`).catch(() => null)

    if (exists) {
      return void res.json(exists)
    }

    const dm = Channel.from({
      type: ChannelTypes.DM,
      recipients: [req.user.id, target.id]
    })

    await dm.save()

    res.json(dm)
  }

  // TODO: Add better relationship handling.
  // @web.post('/:user_id/friend')
  // async friend(req: Request, res: Response): Promise<void> {}
  // @web.route('delete', '/:user_id/friend')
  // async unfriend(req: Request, res: Response): Promise<void> {}
  // @web.post('/:user_id/block')
  // async block(req: Request, res: Response): Promise<void> {}
  // @web.route('delete', '/:user_id')
  // async unblock(req: Request, res: Response): Promise<void> {}
}
