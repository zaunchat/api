
import { Server as T } from '../structures'
import { getaway } from '../getaway'

export class ServerSubscriber implements EventSubscriber<T> {
	async afterCreate({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.subscribe(server.owner.id, [server.id])
		await getaway.publish(server.id, 'SERVER_CREATE', server)
	}

	async afterUpdate({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.publish(server.id, 'SERVER_UPDATE', server)
	}

	async afterDelete({ entity: server }: EventArgs<T>): Promise<void> {
		await getaway.publish(server.id, 'SERVER_DELETE', { id: server.id })

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