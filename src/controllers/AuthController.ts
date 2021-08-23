import { Response, Request } from '@tinyhttp/app'
import * as web from 'express-decorators'
import { User, Session } from '../structures'
import bcrypt from 'bcrypt'
import Validator from 'fastest-validator'

const validator = new Validator()


@web.basePath('/auth')
export default class AuthController {
    checks = {
        login: validator.compile({
            email: { type: 'email' },
            password: { type: 'string' },
            captcha_key: { type: 'string' }
        }),
        logout: validator.compile({
            token: { type: 'string' },
            userId: { type: 'string' }
        }),
        register: validator.compile({
            username: { type: 'string' },
            email: { type: 'email' },
            password: { type: 'string' },
            captcha_key: { type: 'string' }
        })
    }

    @web.post('/login')
    async login(req: Request, res: Response): Promise<void> {
        const valid = this.checks.login(req.body)

		if (valid !== true) {
			return void res.status(400).send(valid)
		}


        const user = await db.findOne(User, {
            email: req.body.email as string
        })

        if (!user) {
            return void res.send('User not exists')
        }

        const isValidPassword = await bcrypt.compare(req.body.password, user.password)

        if (!isValidPassword) {
            return void res.status(403).send('Invalid password')
        }

        const session = Session.from({
            name: req.hostname
        })

        user.sessions.push(session)

        await db.persistAndFlush(user)

        res.send(session)
    }

    @web.post('/logout')
    async logout(req: Request, res: Response): Promise<void> {
        const valid = this.checks.logout(req.body)

		if (valid !== true) {
			return void res.status(400).send(valid)
		}

        const { userId, token } = req.body

        const user = await db.findOne(User, {
            _id: userId,
            sessions: { token }
        })

        if (!user) {
            return void res.send('User not found')
        }

        user.sessions = user.sessions.filter((s) => s.token !== token)

        await db.persistAndFlush(user)

        res.send({ success: true })
    }

    @web.post('/register')
    async register(req: Request, res: Response): Promise<void> {
		const valid = this.checks.register(req.body)

		if (valid !== true) {
			return void res.status(400).send(valid)
		}

        const { username, email, password } = req.body

        const exists = await db.count(User, {
            $or: [{ username }, { email }]
        })

        if (exists) {
            return void res.send('User already exists.')
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

        await db.persistAndFlush(user)

        res.send(session)

    }
}