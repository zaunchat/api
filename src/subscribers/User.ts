
import { User as T } from '../structures'
import { getaway } from '../getaway'

export class UserSubscriber implements EventSubscriber<T> {
	async afterUpdate({ entity: user }: EventArgs<T>): Promise<void> {
		await getaway.publish(user.id, 'USER_UPDATE', {
			id: user.id,
			avatar: user.avatar,
			badges: user.badges,
			username: user.username,
			presence: user.presence
		})
	}

	getSubscribedEntities(): Array<EntityName<T>> {
		return [T]
	}
}