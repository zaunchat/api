import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { Channel } from '../structures'
import { getaway } from '../server'

@Subscriber()
export class ChannelSubscriber<T extends Channel = Channel> implements EventSubscriber<T> {
	async afterCreate({ entity: channel }: EventArgs<T>): Promise<void> {
		await Promise.all(channel.recipients.map((userId) => getaway.subscribe(userId, channel._id)))
		await getaway.publish(channel._id, 'CHANNEL_CREATE', channel)
	}

	async afterUpdate({ entity: channel }: EventArgs<T>): Promise<void> {
		await getaway.publish(channel._id, 'CHANNEL_UPDATE', channel)
	}

	async afterDelete({ entity: channel }: EventArgs<T>): Promise<void> {
		await getaway.publish(channel._id, 'CHANNEL_DELETE', { _id: channel._id })
		await Promise.all(channel.recipients.map((userId) => getaway.unsubscribe(userId, channel._id)))
	}
}