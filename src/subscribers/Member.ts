import { EventArgs, EventSubscriber, EntityName } from '@mikro-orm/core'
import { Member as T, User } from '../structures'
import { getaway } from '../getaway'

export class MemberSubscriber implements EventSubscriber<T> {
  async afterCreate({ entity: member }: EventArgs<T>): Promise<void> {
    await getaway.subscribe(member._id, [member.server._id])
    await getaway.publish(member.server._id, 'MEMBER_JOIN_SERVER', member)

    const user = await User.findOne({ _id: member._id }, { populate: ['servers'] })

    if (user) {
      user.servers.add(member.server)
      await user.save()
    }
  }

  async afterDelete({ entity: member }: EventArgs<T>): Promise<void> {
    await getaway.publish(member.server._id, 'MEMBER_LEAVE_SERVER', {
      _id: member._id,
      server_id: member.server._id
    })

    const user = await User.findOne({ _id: member._id }, { populate: ['servers'] })

    if (user) {
      user.servers.remove(member.server)
      await user.save()
    }
  }

  async afterUpdate({ entity: member }: EventArgs<T>): Promise<void> {
    await getaway.publish(member.server._id, 'MEMBER_UPDATE', member)
  }

  getSubscribedEntities(): Array<EntityName<T>> {
    return [T]
  }
}