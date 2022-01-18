import Redis from 'ioredis'
import config from '@config'

export const createRedisConnection = () => {
	return new Redis(config.database.redis)
}