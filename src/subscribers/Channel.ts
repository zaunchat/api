
import { Channel as T } from '../structures'
import { getaway } from '../getaway'

export class ChannelSubscriber implements EventSubscriber<T> {
	async afterCreate({ entity: channel }: EventArgs<T>): Promise<void> {
		if (channel.recipients) {
			const recipients = channel.recipients.getItems()
			await Promise.all(recipients.map(({ _id }) => getaway.subscribe(_id, [channel.id])))
		}
		
		await getaway.publish(channel.id, 'CHANNEL_CREATE', channel)
	}

	async afterUpdate({ entity: channel }: EventArgs<T>): Promise<void> {
		await getaway.publish(channel.id, 'CHANNEL_UPDATE', channel)
	}

	async afterDelete({ entity: channel }: EventArgs<T>): Promise<void> {
		await getaway.publish(channel.id, 'CHANNEL_DELETE', { id: channel.id })
	}

	getSubscribedEntities(): Array<EntityName<T>> {
		return [T]
	}
}