import { Server } from '../structures'
import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { getaway } from '../server'

@Subscriber()
export class ServerSubscriber<T extends Server = Server> implements EventSubscriber<T> {
	async afterCreate({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.subscribe(server.ownerId, server._id)
		await getaway.publish(server._id, 'SERVER_CREATE', server)
	}

	async afterUpdate({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.publish(server._id, 'SERVER_UPDATE', server)
	}

	async afterDelete({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.publish(server._id, 'SERVER_DELETE', { _id: server._id })
	}
}