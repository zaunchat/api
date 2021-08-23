import 'dotenv/config'

const config = {
	DATABASE_URI: process.env.DATABASE_URI as string,
	PORT: Number(process.env.PORT) || 8080,
	CAPTCHA: {
		ENABLED: false,
		KEY: process.env.CAPTCHA_KEY as string,
		TOKEN: process.env.CAPTCHA_TOKEN as string
	},
	MAX: {
		SERVERS: 100,
		GROUPS: 50,
		SERVER_CHANNELS: 200,
		SERVER_ROLES: 200,
		FRIENDS: 1000,
		MESSAGE_LENGTH: 2000
	}
} as const


export default <T extends keyof typeof config = keyof typeof config>(key: T): typeof config[T] => {
	return config[key]
}