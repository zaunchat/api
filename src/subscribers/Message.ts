
import { Message as T } from '../structures'
import { getaway } from '../getaway'

export class MessageSubscriber implements EventSubscriber<T> {
	async afterCreate({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channel.id, 'MESSAGE_CREATE', message)
	}

	async afterUpdate({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channel.id, 'MESSAGE_UPDATE', message)
	}

	async afterDelete({ entity: message }: EventArgs<T>): Promise<void> {
		await getaway.publish(message.channel.id, 'MESSAGE_DELETE', {
			id: message.id
		})
	}

	getSubscribedEntities(): Array<EntityName<T>> {
		return [T]
	}
}