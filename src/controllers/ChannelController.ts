import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel } from '../structures'

@web.basePath('/channels')
export default class ChannelController {
    @web.get('/:channelId')
    async getChannel(req: Request, res: Response): Promise<void> {
        const channel = await db.em.findOne(Channel, {
            _id: req.body.channelId
        }, ['_id', 'type', 'userId', 'recipients', 'serverId'])

        if (!channel) {
            return void res.sendStatus(404)
        }

        if (!Channel.hasAccess(req.user._id, channel)) {
            return void res.status(403).send('Missing access')
        }

        res.json({
            id: channel._id
        })
    }
}