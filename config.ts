import 'dotenv/config'

const config = {
	database_uri: process.env.DATABASE_URI as string,
	port: Number(process.env.PORT) || 8080,
	smtp: {
		enabled: false,
		uri: process.env.SMTP_URI as string,
	},
	captcha: {
		enabled: false,
		key: process.env.CAPTCHA_KEY as string,
		token: process.env.CAPTCHA_TOKEN as string
	},
	max: {
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