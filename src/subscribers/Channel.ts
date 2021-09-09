import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { Channel as T } from '../structures'
import { getaway } from '../server'

@Subscriber()
export class ChannelSubscriber implements EventSubscriber<T> {
	async afterCreate({ entity: channel }: EventArgs<T>): Promise<void> {
		if (channel.recipients) {
			await Promise.all(channel.recipients.map(({ _id }) => getaway.subscribe(_id, channel._id)))
		}
		await getaway.publish(channel._id, 'CHANNEL_CREATE', channel)
	}

	async afterUpdate({ entity: channel }: EventArgs<T>): Promise<void> {
		await getaway.publish(channel._id, 'CHANNEL_UPDATE', channel)
	}

	async afterDelete({ entity: channel }: EventArgs<T>): Promise<void> {
		await getaway.publish(channel._id, 'CHANNEL_DELETE', { _id: channel._id })
	}
}