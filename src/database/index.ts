import { MikroORM as Database } from 'mikro-orm'
import { Message, User } from '../structures'

export const connect = (clientUrl: string): Promise<Database> => {
	return Database.init({
		clientUrl,
		type: 'mongo',
		entities: [User, Message],
		dbName: 'b9s8hx7mvxwjetc',
		debug: false
	})
}