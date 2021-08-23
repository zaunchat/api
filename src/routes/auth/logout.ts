import { App } from '@tinyhttp/app'
import Validator from 'fastest-validator'
import { User } from '../../structures'

export const LogoutRoute = (app: App, validator: Validator): void => {
    const check = validator.compile({
        token: { type: 'string' },
        userId: { type: 'string' }
    })

    app.post('/auth/logout', async (req, res) => {
        const valid = check(req.body)

        if (valid !== true) {
            return res.status(400).send(valid)
        }

        const { userId, token } = req.body

        const user = await db.em.findOne(User, {
            _id: userId,
            sessions: { token }
        })

        if (!user) {
            return res.send('User not found')
        }

        user.sessions = user.sessions.filter((s) => s.token !== token)

        await db.em.persistAndFlush(user)

        res.send({ success: true })
    })
}