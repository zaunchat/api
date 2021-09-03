import { Group } from '../structures'
import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { getaway } from '../server'

@Subscriber()
export class GroupSubscriber<T extends Group = Group> implements EventSubscriber<T> {
	async afterCreate({ entity: group }: EventArgs<T>): Promise<void> {
		await Promise.all(group.recipients.map((userId) => getaway.subscribe(userId, group._id)))
		await getaway.publish(group._id, 'CHANNEL_CREATE', group)
	}

	async afterUpdate({ entity: group }: EventArgs<T>): Promise<void> {
		await getaway.publish(group._id, 'CHANNEL_UPDATE', group)
	}

	async afterDelete({ entity: group }: EventArgs<T>): Promise<void> {
		await getaway.publish(group._id, 'CHANNEL_DELETE', { _id: group._id })
		await Promise.all(group.recipients.map((userId) => getaway.unsubscribe(userId, group._id)))
	}
}