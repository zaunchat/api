import { AnyEntity, EntityName, EntityRepository, GetRepository, MikroORM } from 'mikro-orm'
import { Channel, Message, User } from '../structures'

class Database {
	private db!: MikroORM
	
	get<T extends AnyEntity<T>, U extends EntityRepository<T> = EntityRepository<T>>(entityName: EntityName<T>): GetRepository<T, U> {
		return this.db.em.getRepository(entityName)
	}

    async connect(clientUrl: string): Promise<this> {
		
		this.db = await MikroORM.init({
			clientUrl,
			type: 'mongo',
			entities: [User, Message, Channel],
			dbName: 'b9s8hx7mvxwjetc',
			debug: false
		})
		

		return this
	}
}

const db = new Database()

export default db