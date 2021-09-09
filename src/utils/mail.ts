import { createTransport } from 'nodemailer'
import { nanoid } from 'nanoid'
import { User } from '../structures'
import isEmail from 'validator/lib/isEmail'
import config from '../../config'


const EMAIL_MESSAGE_TEMPLATE = `Hello @%%USERNAME%%,

You're almost there! If you did not perform this action you can safely ignore this email.
Please verify your account here: %%LINK%%`

interface CreateMailOptions {
	user: User
	title: string
}


class Mail {
	transport = config.smtp.enabled && config.smtp.uri ? createTransport(config.smtp.uri) : null
	queue = new Map<ID, string>()

	get enabled(): boolean  {
		return !!this.transport
	}

	isEmail(address: string): boolean {
		return isEmail(address)
	}

	async send({ user, title }: CreateMailOptions): Promise<string> {
		if (!this.transport) {
			throw new Error('Mail not enabled')
		}

		const token = nanoid(64)
		const link = `${config.endpoints.main}/auth/verify/${user._id}/${token}`

		await this.transport.sendMail({
			from: 'noreply@itchat.com',
			subject: title,
			to: user.email,
			text: EMAIL_MESSAGE_TEMPLATE
				.replace('%%USERNAME%%', user.username)
				.replace('%%LINK%%', link)
		})

		this.queue.set(user._id, token)

		return link
	}

	valid(id: ID, token: string): boolean {
		return this.queue.get(id) === token
	}
}

export const mail = new Mail()