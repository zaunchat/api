import { CacheAdapter } from '@mikro-orm/core'
import Redis from 'ioredis'
import config from '../../config'

export class RedisCacheAdapter implements CacheAdapter {
	db = new Redis(config.database.redis)

	async set(key: string, value: unknown, _origin: string, expiration?: number): Promise<void> {
		const data = JSON.stringify(value)
		if (expiration) {
			await this.db.set(key, data, 'PX', expiration)
		} else {
			await this.db.set(key, data)
		}
	}

	async get(key: string): Promise<string | null> {
		const value = await this.db.get(key)
		return value ? JSON.parse(value) : null
	}

	async clear(): Promise<void> {
		await this.db.flushdb()
	}
}