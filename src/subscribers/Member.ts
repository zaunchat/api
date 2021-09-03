import { Member, User } from '../structures'
import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { getaway } from '../server'

@Subscriber()
export class MemberSubscriber<T extends Member = Member> implements EventSubscriber<T> {
  async afterCreate({ entity: member }: EventArgs<T>): Promise<void> {
    const user = await User.findOne({ _id: member._id })

    user?.servers.push(member.serverId)

    await Promise.all([
      user?.save(),
      getaway.subscribe(member._id, member.serverId)
    ])

    await getaway.publish(member.serverId, 'MEMBER_JOIN_SERVER', member)
  }

  async afterDelete({ entity: member }: EventArgs<T>): Promise<void> {
    const user = await User.findOne({ _id: member._id })

    await user?.save({ servers: user.servers.filter(id => id !== member.serverId) })

    await getaway.publish(member.serverId, 'MEMBER_LEAVE_SERVER', {
      _id: member._id,
      serverId: member.serverId
    })
  }

  async afterUpdate({ entity: { serverId, _id } }: EventArgs<T>): Promise<void> {
    await getaway.publish(serverId, 'MEMBER_UPDATE', { _id, serverId })
  }
}