import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { User, Session } from '../structures'
import { HTTPError } from '../errors'
import { createTransport } from 'nodemailer'
import { nanoid } from 'nanoid'
import { validator } from '../utils'
import bcrypt from 'bcrypt'
import config from '../../config'

const mail = config.smtp.enabled && config.smtp.uri ? createTransport(config.smtp.uri) : null
const waitingForVerify = new Map<string, string>()
const EMAIL_MESSAGE_TEMPLATE = `Hello @%%USERNAME%%,

You're almost there! If you did not perform this action you can safely ignore this email.
Please verify your account here: %%LINK%%`

@web.basePath('/auth')
export class AuthController {
    checks = {
        login: validator.compile({
            email: { type: 'email' },
            password: { type: 'string' }
        }),
        logout: validator.compile({
            token: { type: 'string' },
            userId: { type: 'string' }
        }),
        register: validator.compile({
            username: { type: 'string' },
            email: { type: 'email' },
            password: { type: 'string' }
        })
    }

    @web.get('/check')
    async check(req: Request, res: Response): Promise<void> {
        const token = req.headers['x-session-token']
        const userId = req.headers['x-session-id']

        const user = token && userId ? await User.findOne({
            _id: userId,
            deleted: false,
            verified: true
        }, {
            fields: ['sessions']
        }) : null

        res.json({
            valid: !!user?.sessions.some((session) => session.token === token)
        })
    }


    @web.post('/login')
    async login(req: Request, res: Response): Promise<void> {
        const valid = this.checks.login(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const { email, password } = req.body

        const user = await User.findOne({ email })

        if (!user) {
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        if (!await bcrypt.compare(password, user.password)) {
            return void res.status(403).send(new HTTPError('INVALID_PASSWORD'))
        }

        if (!user.verified) {
            return void res.status(403).send(new HTTPError('USER_NOT_VERIFIED'))
        }

        const session = Session.from({
            name: req.hostname
        })

        await user.save({ sessions: [...user.sessions, session] })

        res.json({
            token: session.token,
            id: user._id
        })
    }

    @web.post('/logout')
    async logout(req: Request, res: Response): Promise<void> {
        const valid = this.checks.logout(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const { userId, token } = req.body

        const user = await User.findOne({
            _id: userId,
            deleted: false
        })

        if (!user) {
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        const oldSessionsSize = user.sessions.length

        user.sessions = user.sessions.filter((s) => s.token !== token)

        if (oldSessionsSize === user.sessions.length) {
            return void res.status(404).send(new HTTPError('UNKNOWN_SESSION'))
        }

        await user.save()

        res.sendStatus(202)
    }

    @web.post('/register')
    async register(req: Request, res: Response): Promise<void> {
        const valid = this.checks.register(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        const { username, email, password } = req.body

        const exists = await User.findOne({
            $or: [{ username }, { email }]
        })

        if (exists) {
            if (username === exists.username) {
                return void res.status(404).send(new HTTPError('USERNAME_TAKEN'))
            } else {
                return void res.status(404).send(new HTTPError('EMAIL_ALREADY_IN_USE'))
            }
        }

        const user = await User.from({
            username,
            email,
            password: await bcrypt.hash(password, 12)
        }).save({ verified: !mail })

        if (!mail) {
            return void res.redirect(`https://${req.headers.host}/auth/login`)
        }

        const token = nanoid(50)
        const link = `https://${req.headers.host}/auth/verify/${user._id}/${token}`

        try {

            await mail.sendMail({
                subject: 'Verify your Itchat account.‏‏',
                from: 'noreply@itchat.com',
                to: user.email,
                text: EMAIL_MESSAGE_TEMPLATE
                    .replace('%%USERNAME%%', user.username)
                    .replace('%%LINK%%', link)
            })

            waitingForVerify.set(user._id, token)

            res.json({ link })
        } catch (err) {
            console.error(err)
            await User.remove(user)
            res.sendStatus(500)
        }
    }

    @web.get('/verify/:userId/:token')
    async verify(req: Request, res: Response): Promise<void> {
        const { userId, token } = req.params

        if (token !== waitingForVerify.get(userId)) {
            return void res.status(404).send(new HTTPError('UNKNOWN_TOKEN'))
        }

        const user = await User.findOne({
            _id: userId
        })

        if (!user) {
            return void res.status(404).send(new HTTPError('UNKNOWN_USER'))
        }

        await user.save({ verified: true })

        waitingForVerify.delete(userId)

        res.redirect(`https://${req.headers.host}/auth/login`)
    }
}