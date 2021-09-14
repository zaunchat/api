import { EventArgs, EventSubscriber, EntityName } from '@mikro-orm/core'
import { Server as T } from '../structures'
import { getaway } from '../server'

export class ServerSubscriber implements EventSubscriber<T> {
	async afterCreate({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.subscribe(server.owner._id, server._id)
		await getaway.publish(server._id, 'SERVER_CREATE', server)
	}

	async afterUpdate({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.publish(server._id, 'SERVER_UPDATE', server)
	}

	async afterDelete({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.publish(server._id, 'SERVER_DELETE', { _id: server._id })

		const promises: Promise<unknown>[] = []

		for (const channel of server.channels) {
			promises.push(channel.delete())
		}

		for (const role of server.roles) {
			promises.push(role.delete())
		}

		await Promise.all(promises)
	}

	getSubscribedEntities(): Array<EntityName<T>> {
		return [T]
	}
}