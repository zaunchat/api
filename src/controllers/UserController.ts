import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, User } from '../structures'
import { array } from 'pg-query-config'

@web.basePath('/users')
export class UserController {
  @web.get('/:user_id')
  async fetchOne(req: Request, res: Response): Promise<void> {
    const user = await User.fetchPublicUser(req.params.user_id as ID)
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
    const target = await User.fetchPublicUser(user_id)
    const exists = await Channel.findOne({
      type: ChannelTypes.DM,
      recipients: array.lc([user_id])
    }).catch(() => null)

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
