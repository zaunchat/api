import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Server, Channel, CreateServerSchema, Member, ChannelTypes } from '../../structures'
import { HTTPError } from '../../errors'
import config from '../../../config'
import db from '../../database'


@web.basePath('/servers')
export class ServerController {
    @web.get('/')
    async fetchServers(req: Request, res: Response): Promise<void> {
        const servers = await Server.find({
            _id: {
                $in: req.user.servers
            }
        })
        res.json(servers)
    }

    @web.get('/:serverId')
    async fetchServer(req: Request, res: Response): Promise<void> {
        if (!req.user.servers.some(id => id === req.params.serverId)) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const server = await Server.findOne({
            _id: req.params.serverId
        })

        if (!server) {
            throw new HTTPError('UNKNOWN_SERVER')
        }

        res.json(server)
    }

    @web.post('/')
    async createServer(req: Request, res: Response): Promise<void> {
        req.check(CreateServerSchema)

        if (req.user.servers.length >= config.limits.user.servers) {
            throw new HTTPError('MAXIMUM_SERVERS')
        }

        const server = Server.from({
            ...req.body,
            ownerId: req.user._id
        })

        const chat = Channel.from({
            type: ChannelTypes.TEXT,
            serverId: server._id,
            name: 'general'
        })

        const category = Channel.from({
            type: ChannelTypes.CATEGORY,
            name: 'General',
            serverId: server._id,
            channels: [chat._id]
        })

        const member = Member.from({
            _id: req.user._id,
            serverId: server._id
        })

        await db.save([
            server,
            chat,
            category,
            member
        ])

        res.json(server)
    }
}