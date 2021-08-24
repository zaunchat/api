import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel } from '../structures'

@web.basePath('/channels')
export default class ChannelController {
    @web.get('/:channelId')
    async getChannel(req: Request, res: Response): Promise<void> {
        const channel = await db.findOne(Channel, {
            _id: req.body.channelId
        })

        if (!channel) {
            return void res.sendStatus(404)
        }

        if (!Channel.hasAccess(req.user._id, channel)) {
            return void res.status(403).send('Missing access')
        }

        res.json(Channel.toObject(channel))
    }
}