import { Message } from '../structures'
import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { getaway } from '../server'

@Subscriber()
export class MessageSubscriber<T extends Message = Message> implements EventSubscriber<T> {
	async afterCreate({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channelId, 'MESSAGE_CREATE', message)
	}

	async afterUpdate({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channelId, 'MESSAGE_UPDATE', message)
	}

	async afterDelete({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channelId, 'MESSAGE_DELETE', {
			_id: message._id,
			channelId: message.channelId
		})
	}
}