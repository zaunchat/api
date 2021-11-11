import 'dotenv/config'
import env from 'env-var'

const config = {
	port: env.get('PORT').default(8080).asPortNumber(),
	database: {
		name: env.get('DATABASE_NAME').asString(),
		uri: env.get('DATABASE_URI').required().asUrlString(),
		type: env.get('DATABASE_TYPE').default('mongo').asString(), // mongo | mysql | mariadb | postgresql | sqlite
		redis: env.get('REDIS_URI').asString()
	},
	smtp: {
		enabled: env.get('SMTP_ENABLED').default('true').asBool(),
		uri: env.get('SMTP_URI').required().asUrlString()
	},
	captcha: {
		enabled: env.get('CATPCHA_ENABLED').default('true').asBool(),
		key: env.get('CAPTCHA_KEY').asString(),
		token: env.get('CAPTCHA_TOKEN').asString()
	},
	limits: {
		user: {
			username: 32,
			servers: 100,
			groups: 50,
			friends: 1_000,
			blocked: 1_000,
		},
		server: {
			name: 50,
			description: 1_000,
			roles: 200,
			channels: 200,
			emojis: 50,
			members: 10_000
		},
		member: {
			nickname: 32
		},
		message: {
			length: 2_000,
			attachments: 5,
			replies: 5,
			embeds: 1
		},
		group: {
			name: 50,
			members: 100,
			description: 1_000
		},
		channel: {
			name: 50,
			topic: 1000
		}
	},
	endpoints: {
		main: env.get('DOMAIN').default('https://itchat.com').asUrlString(),
		app: env.get('APP_DOMAIN').default('https://app.itchat.world').asUrlString(),
		api: env.get('API_DOMAIN').default('https://api.itchat.world').asUrlString(),
		cdn: env.get('CDN_DOMAIN').default('https://cdn.itchat.world').asUrlString()
	}
} as const


export default config