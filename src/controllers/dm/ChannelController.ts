import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { DMChannel } from '../../structures'
import { HTTPError } from '../../errors'
import { getaway } from '../../server'
import { BASE_CHANNEL_PATH } from '.'


@web.basePath(BASE_CHANNEL_PATH)
export class ChannelController {
    @web.get('/')
    async fetchDMs(req: Request, res: Response): Promise<void> {
        const channels = await DMChannel.find({
            recipients: req.user._id,
            deleted: false
        })
        res.json(channels)
    }


    @web.get('/:channelId')
    async fetchDM(req: Request, res: Response): Promise<void> {
        const dm = await DMChannel.findOne({
            _id: req.params.channelId,
            recipients: req.user._id,
            deleted: false
        })

        if (!dm) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        res.json(dm)
    }



    @web.route('delete', '/:channelId')
    async deleteDM(req: Request, res: Response): Promise<void> {
        const dm = await DMChannel.findOne({
            _id: req.params.channelId,
            recipients: req.user._id,
            deleted: false
        })

        if (!dm) {
            throw new HTTPError('UNKNOWN_CHANNEL')
        }

        await dm.save({ deleted: true })

        getaway.publish(dm._id, 'CHANNEL_DELETE', { _id: dm._id })

        res.ok()
    }
}