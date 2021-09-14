import { Configuration, Options } from '@mikro-orm/core'
import * as Subscribers from '../subscribers'
import { User, Message, Channel, Server, Session, Member, Role, Invite } from '../structures'
import { RedisCacheAdapter } from './redis'
import config from '../../config'
import ms from 'ms'


const options: Options = {
	clientUrl: config.database.uri,
	type: config.database.type as keyof typeof Configuration.PLATFORMS,
	entities: [User, Message, Channel, Server, Session, Member, Role, Invite],
	subscribers: Object.values(Subscribers).map(Subscriber => new Subscriber()),
	dbName: config.database.name,
	debug: false,
	cache: {
		enabled: true,
		adapter: RedisCacheAdapter
	},
	resultCache: {
		adapter: RedisCacheAdapter,
		expiration: ms('5 second')
	}
}

export default options