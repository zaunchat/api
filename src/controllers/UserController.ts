import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel, ChannelTypes, User, RelationshipStatus } from '../structures'
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
    const { user_id } = req.params
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


  @web.use('/@me/relationships/:user_id')
  async relationships(req: Request, res: Response): Promise<void> {
    if (req.params.user_id === req.user.id) {
      req.throw('MISSING_ACCESS')
    }

    const target = await User.findOne({ id: req.params.user_id })
    const relations = req.user.relations, targetRelations = target.relations


    switch (req.method) {
      case 'POST': // Add friend
        if (relations[target.id] === RelationshipStatus.FRIEND) {
          req.throw('ALREADY_FRIENDS')
        } else if (relations[target.id] === RelationshipStatus.OUTGOING) {
          req.throw('ALREADY_SENT_REQUEST')
        } else if (target.relations[req.user.id] === RelationshipStatus.OUTGOING) { // Now friends!
          targetRelations[req.user.id] = relations[target.id] = RelationshipStatus.FRIEND
        } else { // Sent request
          relations[target.id] = RelationshipStatus.OUTGOING
          targetRelations[req.user.id] = RelationshipStatus.IN_COMING
        }
        break
      case 'PUT': // Block
        if (relations[target.id] === RelationshipStatus.BLOCKED) {
          req.throw('BLOCKED')
        }
        relations[target.id] = RelationshipStatus.BLOCKED
        targetRelations[req.user.id] = RelationshipStatus.BLOCKED_OTHER
        break
      case 'DELETE': // Unfriend or unblock
        if (!(target.id in relations)) {
          // req.throw('NOT_EXISTS')
        }

        delete relations[target.id]
        break
      default:
        break
    }


    await Promise.all([
      req.user.update({ relations }),
      target.update({ relations: targetRelations })
    ])


    res.sendStatus(202)
  }
}
