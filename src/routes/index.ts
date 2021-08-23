import { App } from '@tinyhttp/app'
import Validator from 'fastest-validator'
import config from '../../config'
import { User } from '../structures'
import { Captcha } from '../utils/Captcha'
import * as AuthRoutes from './auth'

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

export const validator = new Validator()
export type RouteLike = (api: App, validator: Validator) => void

const register = (routes: Record<string, RouteLike> | RouteLike[]): void => {
	if (!Array.isArray(routes)) routes = Object.values(routes)
	for (const route of routes) route(app, validator)
}

register(AuthRoutes)


export default app