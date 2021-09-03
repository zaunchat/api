import { AnyEntity, EntityName, EntityRepository, GetRepository, MikroORM, Configuration } from 'mikro-orm'
import { Message, User, Server, Member, Channel, DMChannel, TextChannel, Group } from '../structures'
import * as Subscribers from '../subscribers'
import { RedisCacheAdapter } from './redis'
import config from '../../config'


class Database extends MikroORM {
	get<T extends AnyEntity<T>, U extends EntityRepository<T> = EntityRepository<T>>(entityName: EntityName<T>): GetRepository<T, U> {
		return this.em.getRepository(entityName)
	}

	save<T extends AnyEntity<T>>(entity: T | T[]) {
		return this.em.persistAndFlush(entity)
	}
}

const db = new Database({
	clientUrl: config.database.uri,
	type: config.database.type as keyof typeof Configuration.PLATFORMS,
	entities: [User, Message, Server, Member, DMChannel, TextChannel, Group, Channel],
	subscribers: Object.values(Subscribers).map(Subscriber => new Subscriber()),
	dbName: 'b9s8hx7mvxwjetc',
	debug: false,
	resultCache: {
		adapter: RedisCacheAdapter
	}
})

export default db