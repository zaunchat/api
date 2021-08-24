import { App } from '@tinyhttp/app'
import config from '../../config'
import { User } from '../structures'
import { Captcha } from '../utils/Captcha'
import { register } from 'express-decorators'
import AuthController from './AuthController'
import ChannelController from './ChannelController'
import UserController from './UserController'
import { HTTPError } from '../errors'

export const app = new App()

const NON_AUTH_ROUTES = ['login', 'register', 'verify'].map((r) => '/auth/' + r)

app.use(async (req, res, next) => {
	if (NON_AUTH_ROUTES.includes(req.path)) {
		if (config('CAPTCHA').ENABLED) {
			const captchaChecked = req.body.captcha_key && await Captcha.check(req.body.captcha_key)
			if (!captchaChecked) {
				return res.status(403).send(new HTTPError('FIELD_CAPTCHA'))
			}
		}
		return next()
	}

	const token = req.headers.authorization
	const user = token ? await db.findOne(User, {
		sessions: { token }
	}) : null

	if (!user) {
		return res.status(401).send(new HTTPError('UNAUTHORIZED'))
	}

	Object.defineProperty(req, 'user', {
		value: user
	})

	next()
})

register(app, new AuthController())
register(app, new UserController())
register(app, new ChannelController())

export default app