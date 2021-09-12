import { Configuration, Options } from '@mikro-orm/core'
import { TsMorphMetadataProvider } from '@mikro-orm/reflection'
import * as Subscribers from '../subscribers'
import { RedisCacheAdapter } from './redis'
import config from '../../config'
import ms from 'ms'

const entities = ['Message', 'User', 'Server', 'Member', 'Channel', 'Role', 'Invite'].join(',')


const options: Options = {
	clientUrl: config.database.uri,
	type: config.database.type as keyof typeof Configuration.PLATFORMS,
	entities: [`./dist/src/structures/{${entities}}.js`],
	entitiesTs: [`./src/structures/{${entities}}.ts`],
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
	},
	metadataProvider: TsMorphMetadataProvider
}

export default options