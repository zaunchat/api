import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { User, Session, CreateUserSchema, LoginUserSchema, LogoutUserSchema } from '../structures'
import { mail } from '../utils'
import { HTTPError } from '../errors'
import argon2 from 'argon2'


@web.basePath('/auth')
export class AuthController {
    @web.get('/check')
    async check(req: Request, res: Response): Promise<void> {
        const token = req.headers['x-session-token']
        const userId = req.headers['x-session-id']

        const user = token && userId ? await User.findOne({
            _id: userId,
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

        user.sessions.push(session)

        await user.save()

        res.json({
            token: session.token,
            id: user._id
        })
    }

    @web.post('/logout')
    async logout(req: Request, res: Response): Promise<void> {
        req.check(LogoutUserSchema)

        const { userId, token } = req.body

        const user = await User.findOne({
            _id: userId
        })

        if (!user) {
            throw new HTTPError('UNKNOWN_USER')
        }

        const oldSessionsSize = user.sessions.length

        user.sessions = user.sessions.filter((s) => s.token !== token)

        if (oldSessionsSize === user.sessions.length) {
            throw new HTTPError('UNKNOWN_SESSION')
        }

        await user.save()

        res.ok()
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
            return void res.redirect(`https://${req.headers.host}/auth/login`)
        }

        try {
            res.json({
                url: await mail.send({
                    title: 'Verify your Itchat account.‏‏',
                    user
                })
            })
        } catch (err) {
            await User.remove(user)
            throw err
        }
    }

    @web.get('/verify/:userId/:token')
    async verify(req: Request, res: Response): Promise<void> {
        const { userId, token } = req.params as { userId: Snowflake; token: string }

        if (!mail.valid(userId, token)) {
            throw new HTTPError('UNKNOWN_TOKEN')
        }

        mail.queue.delete(userId)

        const user = await User.findOne({
            _id: userId
        })

        if (!user) {
            throw new HTTPError('UNKNOWN_USER')
        }

        await user.save({ verified: true })

        res.redirect(`https://${req.headers.host}/auth/login`)
    }
}