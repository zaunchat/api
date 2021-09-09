import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { Member as T } from '../structures'
import { getaway } from '../server'

@Subscriber()
export class MemberSubscriber implements EventSubscriber<T> {
  async afterCreate({ entity: member }: EventArgs<T>): Promise<void> {
    await getaway.subscribe(member._id, member.server._id)
    await getaway.publish(member.server._id, 'MEMBER_JOIN_SERVER', member)
  }

  async afterDelete({ entity: member }: EventArgs<T>): Promise<void> {
    // TODO:
    // const user = await User.findOne({ _id: member._id })

    // await user?.save({ servers: user.servers.filter(id => id !== member.server_id) })

    await getaway.publish(member.server._id, 'MEMBER_LEAVE_SERVER', {
      _id: member._id,
      server_id: member.server._id
    })
  }

  async afterUpdate({ entity: member }: EventArgs<T>): Promise<void> {
    await getaway.publish(member.server._id, 'MEMBER_UPDATE', member)
  }
}