import { AnyEntity, EntityName, EntityRepository, GetRepository, MikroORM } from '@mikro-orm/core'
import config from './config'

class Database {
	orm!: MikroORM

	get<T extends AnyEntity<T>, U extends EntityRepository<T> = EntityRepository<T>>(entityName: EntityName<T>): GetRepository<T, U> {
		return this.orm.em.getRepository(entityName)
	}

	save<T extends AnyEntity<T>>(entity: T | T[]): Promise<void> {
		return this.orm.em.persistAndFlush(entity)
	}

	async connect(): Promise<this> {
		this.orm = await MikroORM.init(config)
		return this
	}
}

const db = new Database()

export default db