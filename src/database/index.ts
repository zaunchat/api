import { AnyEntity, EntityName, EntityRepository, GetRepository, MikroORM } from 'mikro-orm'
import config from './config'

class Database {
	db!: MikroORM

	get<T extends AnyEntity<T>, U extends EntityRepository<T> = EntityRepository<T>>(entityName: EntityName<T>): GetRepository<T, U> {
		return this.db.em.getRepository(entityName)
	}

	save<T extends AnyEntity<T>>(entity: T | T[]): Promise<void> {
		return this.db.em.persistAndFlush(entity)
	}

	async connect(): Promise<this> {
		this.db = await MikroORM.init(config)
		return this
	}
}

const db = new Database()

export default db