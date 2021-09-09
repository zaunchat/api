import { EventArgs, EventSubscriber, Subscriber } from '@mikro-orm/core'
import { User as T } from '../structures'
import { getaway } from '../server'

@Subscriber()
export class UserSubscriber implements EventSubscriber<T> {
	async afterUpdate({ entity: user }: EventArgs<T>): Promise<void> {
		await getaway.publish(user._id, 'USER_UPDATE', {
			_id: user._id,
			avatar: user.avatar,
			badges: user.badges,
			username: user.username,
			presence: user.presence
		})
	}
}