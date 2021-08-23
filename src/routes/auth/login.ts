import type { App } from '@tinyhttp/app'
import type Validator from 'fastest-validator'
import bcrypt from 'bcrypt'
import { Session, User } from '../../structures'

export const LoginRoute = (app: App, validator: Validator): void => {
	const check = validator.compile({
		email: { type: 'email' },
		password: { type: 'string' },
		captcha_key: { type: 'string' }
	})

	app.post('/login', async (req, res) => {
		const valid = check(req.body)

		if (valid !== true) {
			return res.status(400).send(valid)
		}

		const user = await db.em.findOne(User, {
			email: req.body.email as string
		})

		if (!user) {
			return res.send('User not exists')
		}

		const isValidPassword = await bcrypt.compare(req.body.password, user.password)

		if (!isValidPassword) {
			return res.status(403).send('Invalid password')
		}

		const session = Session.from({
			name: req.hostname
		})

		user.sessions.push(session)

		await db.em.persistAndFlush(user)

		res.send(session)
	})
}
