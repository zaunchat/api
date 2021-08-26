import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Channel } from '../structures'
import { HTTPError } from '../errors'
import db from '../database'

@web.basePath('/channels')
export class ChannelController {
    @web.get('/:channelId')
    async fetchChannel(req: Request, res: Response): Promise<void> {
        const channel = await db.get(Channel).findOne({
            _id: req.body.channelId  
        })

        if (!channel) {
            return void res.status(404).send(new HTTPError('UNKNOWN_CHANNEL'))
        }

        if (!Channel.hasAccess(req.user._id, channel)) {
            return void res.status(403).send(new HTTPError('MISSING_ACCESS'))
        }

        res.json(Channel.toObject(channel))
    }
}