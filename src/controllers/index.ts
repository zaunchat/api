import { App } from '@tinyhttp/app'
import config from '../../config'
import { User } from '../structures'
import { Captcha } from '../utils/Captcha'
import { register } from 'express-decorators'
import AuthController from './AuthController'
import ChannelController from './ChannelController'
import UserController from './UserController'

export const app = new App()


app.use(async (req, res, next) => {
	if (req.method === 'POST' && ['/login', '/register'].includes(req.path)) {

		if (config('CAPTCHA').ENABLED) {
			const captchaChecked = req.body.captcha_key && await Captcha.check(req.body.captcha_key)
			if (!captchaChecked) {
				return res.sendStatus(403)
			}
		}

		return next()
	}

	const token = req.headers.authorization
	const user = token ? await db.em.findOne(User, {
		sessions: { token }
	}) : null

	if (!user) {
		return res.sendStatus(401)
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