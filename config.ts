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
			servers: 100,
			groups: 50,
			friends: 1000,
			blocked: 1000
		},
		server: {
			roles: 200,
			channels: 200,
			emojis: 50
		},
		message: {
			length: 2000,
			attachments: 5,
			replies: 5,
			embeds: 1
		}
	}
} as const

export default config