import { Response, Request } from '@tinyhttp/app'
import * as web from 'express-decorators'
import { User, Session } from '../structures'
import bcrypt from 'bcrypt'
import Validator from 'fastest-validator'
import { HTTPError } from '../errors'
import { createTransport } from 'nodemailer'
import config from '../../config'
import { nanoid } from 'nanoid'

const mail = createTransport(config('SMTP_URI'))


const validator = new Validator()
const waitingForVerify = new Map<string, string>()

const EMAIL_TEMPLATE = `
Hello @%%USERNAME%%,

You're almost there! If you did not perform this action you can safely ignore this email.
Please verify your account here: %%LINK%%
`

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
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        if (!await bcrypt.compare(req.body.password, user.password)) {
            return void res.status(403).send(new HTTPError('INVALID_PASSWORD'))
        }

        if (!user.verified) {
            return void res.status(403).send(new HTTPError('USER_NOT_VERIFIED'))
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
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        const oldSessionsSize = user.sessions.length

        user.sessions = user.sessions.filter((s) => s.token !== token)

        if (oldSessionsSize === user.sessions.length) {
            return void res.status(404).send(new HTTPError('UNKNOWN_SESSION'))
        }

        await db.persistAndFlush(user)

        res.sendStatus(202)
    }

    @web.post('/register')
    async register(req: Request, res: Response): Promise<void> {
        const valid = this.checks.register(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const { username, email, password } = req.body

        const exists = await db.findOne(User, {
            $or: [{ username }, { email }]
        })

        if (exists) {
            if (username === exists.username) {
                return void res.status(404).send(new HTTPError('USERNAME_TAKEN'))
            } else {
                return void res.status(404).send(new HTTPError('EMAIL_ALREADY_IN_USE'))
            }
        }

        await db.persistAndFlush(User.from({
            username,
            email,
            password: await bcrypt.hash(password, 12)
        }))


        const user = await db.findOneOrFail(User, {
            username,
            email
        })


        const token = nanoid(50)
        const link = `https://${req.headers.host}/auth/verify/${user._id}/${token}`

        try {

            await mail.sendMail({
                subject: 'Verify your Itchat account.‏‏',
                from: 'noreply@itchat.com',
                to: user.email,
                text: EMAIL_TEMPLATE
                    .replace('%%USERNAME%%', user.username)
                    .replace('%%LINK%%', link)
            })

            waitingForVerify.set(user._id, token)

            res.json({ link })
        } catch (err) {
            console.error(err)
            await db.removeAndFlush(user)
            res.sendStatus(500)
        }
    }

    @web.get('/verify/:userId/:token')
    async verify(req: Request, res: Response): Promise<void> {
        const { userId, token } = req.params

        if (token !== waitingForVerify.get(userId)) {
            return void res.status(404).send(new HTTPError('UNKNOWN_TOKEN'))
        }

        const user = await db.findOne(User, {
            _id: userId,
            verified: false
        })

        if (!user) {
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        user.verified = true

        await db.persistAndFlush(user)

        waitingForVerify.delete(userId)

        res.redirect(`https://${req.headers.host}/auth/login`)
    }
}