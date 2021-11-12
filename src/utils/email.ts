import { createTransport } from 'nodemailer'
import { nanoid } from 'nanoid'
import { User } from '../structures'
import config from '../config'
import { createRedisConnection } from '../database/redis'

const THREE_HOURS = 10_800_000
const EMAIL_MESSAGE_TEMPLATE = `Hello @%%USERNAME%%,

You're almost there! If you did not perform this action you can safely ignore this email.
Please verify your account here: %%LINK%%`


class Email {
	redis = createRedisConnection()
	readonly client = config.smtp.enabled && config.smtp.uri ? createTransport(config.smtp.uri) : null

	get enabled(): boolean  {
		return !!this.client
	}

	static generateCode(): string {
		return nanoid(64)
	}

	async send(user: User): Promise<string> {
		if (!this.client) {
			throw new Error('Email not enabled')
		}

		const code = Email.generateCode()
		const link = `${config.endpoints.main}/auth/verify/${user._id}/${code}`

		await this.client.sendMail({
			from: 'noreply@itchat.com',
			subject: 'Verify your email',
			to: user.email,
			text: EMAIL_MESSAGE_TEMPLATE
				.replace('%%USERNAME%%', user.username)
				.replace('%%LINK%%', link)
		})


		// Expires after three hours.
		await this.redis.set(user._id, code, 'PX', THREE_HOURS)

		return link
	}

	async verify(key: ID, code: string): Promise<boolean> {
		const exists = await this.redis.get(key)

		if (exists && exists === code) {
			
			// Allow to use one time
			await this.redis.del(key)

			return true
		}

		return false
	}
}

export const email = new Email()