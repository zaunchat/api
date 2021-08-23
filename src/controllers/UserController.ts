import { Response, Request } from '@tinyhttp/app'
import * as web from 'express-decorators'
import { User } from '../structures'

@web.basePath('/users')
export default class UserController {
    @web.get('/:userId')
    async getUser(req: Request, res: Response): Promise<void> {
        const { userId } = req.params

        const user = await db.em.findOne(User, {
            _id: userId === '@me' ? req.user._id : userId
        }, ['username', 'avatar', 'badges', '_id'])

        if (!user) {
            return void res.sendStatus(404)
        }

        res.json(user)
    }
}