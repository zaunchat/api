import { Connection, IDatabaseDriver, MikroORM as Database, MikroORM } from 'mikro-orm'
import { Message, User } from '../structures'

export type DBConnection = MikroORM<IDatabaseDriver<Connection>>

export const connect = (clientUrl: string): Promise<DBConnection> => {
	return Database.init({
		clientUrl,
		type: 'mongo',
		entities: [User, Message],
		dbName: 'b9s8hx7mvxwjetc',
		debug: false
	})
}