import type { App } from '@tinyhttp/app'
import type Validator from 'fastest-validator'
import { User, Session } from '../../structures'
import bcrypt from 'bcrypt'

export default (app: App, validator: Validator): void => {
	const check = validator.compile({
		username: { type: 'string' },
		email: { type: 'email' },
		password: { type: 'string' },
		captcha_key: { type: 'string' }
	})

	app.post('/register', async (req, res) => {
		const valid = check(req.body)

		if (valid !== true) {
			return res.status(400).send(valid)
		}

		const { username, email, password } = req.body

		const exists = await db.em.count(User, {
			$or: [{ username }, { email }]
		})

		if (exists) {
			return res.send('User already exists.')
		}

		const user = User.from({
			username,
			email,
			password: await bcrypt.hash(password, 12)
		})

		const session = Session.from({
			name: req.hostname
		})

		user.sessions.push(session)

		await db.em.persistAndFlush(user)

		res.send(session)
	})
}