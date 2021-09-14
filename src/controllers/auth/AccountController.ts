import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { User, Session, CreateUserSchema, LoginUserSchema } from '../../structures'
import { HTTPError } from '../../errors'
import { mail } from '../../utils'
import config from '../../../config'
import argon2 from 'argon2'



@web.basePath('/auth/accounts')
export class AccountController {
	@web.post('/login')
	async login(req: Request, res: Response): Promise<void> {
		req.check(LoginUserSchema)

		const { email, password } = req.body

		if (!mail.isEmail(email)) {
			throw new HTTPError('INVALID_EMAIL')
		}

		const user = await User.findOne({ email })

		if (!user) {
			throw new HTTPError('UNKNOWN_USER')
		}

		if (!user.verified) {
			throw new HTTPError('USER_NOT_VERIFIED')
		}

		if (!await argon2.verify(password, user.password)) {
			throw new HTTPError('INVALID_PASSWORD')
		}

		const session = Session.from({
			name: req.hostname
		})

		user.sessions.add(session)

		await user.save()

		res.json({
			token: session.token,
			id: user._id
		})
	}

	@web.post('/register')
	async register(req: Request, res: Response): Promise<void> {
		req.check(CreateUserSchema)

		const { username, email, password } = req.body

		if (!mail.isEmail(email)) {
			throw new HTTPError('INVALID_EMAIL')
		}

		const exists = await User.findOne({
			$or: [{ username }, { email }]
		}, {
			fields: ['username', 'email']
		})

		if (exists) {
			if (username === exists.username) {
				throw new HTTPError('USERNAME_TAKEN')
			} else {
				throw new HTTPError('EMAIL_ALREADY_IN_USE')
			}
		}

		const user = await User.from({
			username,
			email,
			password: await argon2.hash(password)
		}).save({ verified: !mail.enabled })

		if (!mail.enabled) {
			return void res.redirect(`https://${config.endpoints.main}/auth/login`)
		}

		try {
			res.json({
				url: await mail.send({ title: 'Verify your Itchat account.‏‏', user })
			})
		} catch (err) {
			await User.remove(user)
			throw err
		}
	}

	@web.get('/verify/:user_id/:token')
	async verify(req: Request, res: Response): Promise<void> {
		const { user_id, token } = req.params as { user_id: ID; token: string }

		if (!mail.valid(user_id, token)) {
			throw new HTTPError('UNKNOWN_TOKEN')
		}

		mail.queue.delete(user_id)

		const user = await User.findOne({ _id: user_id })

		if (!user) {
			throw new HTTPError('UNKNOWN_USER')
		}

		await user.save({ verified: true })

		res.redirect(`${config.endpoints.main}/auth/login`)
	}
}