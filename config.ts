import 'dotenv/config'

const config = {
	port: Number(process.env.PORT) || 8080,
	database: {
		uri: process.env.DATABASE_URI as string,
		type: process.env.DATABASE_TYPE || 'mongo' // mongo | mysql | mariadb | postgresql | sqlite
	},
	redis: {
		uri: process.env.REDIS_URI as string,
		local: true
	},
	smtp: {
		enabled: false,
		uri: process.env.SMTP_URI as string,
	},
	captcha: {
		enabled: false,
		key: process.env.CAPTCHA_KEY as string,
		token: process.env.CAPTCHA_TOKEN as string
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
	routes: {
		global: '20/5s',
		'auth/check': '30/5m --ip',
		'auth/login': '3/24h --ip',
		'auth/register': '3/24h --ip',
		'auth/verify': '2/24h --ip',
		servers: '5/5s',
		groups: '5/5s',
		channels: '5/5s',
		users: '5/5s'
	}
} as const

export default config