import { AnyEntity, EntityName, EntityRepository, GetRepository, MikroORM } from 'mikro-orm'
import { Message, User, Server, Member, Channel, DMChannel } from '../structures'
import config from '../../config'

class Database {
	private db!: MikroORM

	get<T extends AnyEntity<T>, U extends EntityRepository<T> = EntityRepository<T>>(entityName: EntityName<T>): GetRepository<T, U> {
		return this.db.em.getRepository(entityName)
	}

	async connect(): Promise<this> {

		this.db = await MikroORM.init({
			clientUrl: config.database_uri,
			type: 'mongo',
			entities: [User, Message, Server, Member, DMChannel, Channel],
			dbName: 'b9s8hx7mvxwjetc',
			debug: false
		})


		return this
	}
}

const db = new Database()

export default db