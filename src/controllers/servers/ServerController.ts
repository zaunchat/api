import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Server, Channel, CreateServerSchema, Member, ChannelTypes } from '../../structures'
import { HTTPError } from '../../errors'
import config from '../../../config'
import db from '../../database'


@web.basePath('/servers')
export class ServerController {
    @web.get('/')
    async fetchMany(req: Request, res: Response): Promise<void> {
        res.json(req.user.servers.getItems())
    }

    @web.get('/:server_id')
    async fetchOne(req: Request, res: Response): Promise<void> {
        const { server_id } = req.params as { server_id: ID }

        if (!req.user.servers.getItems().some(server => server._id === server_id)) {
            throw new HTTPError('MISSING_ACCESS')
        }

        const server = await Server.findOne({
            _id: server_id
        })

        if (!server) {
            throw new HTTPError('UNKNOWN_SERVER')
        }

        res.json(server)
    }

    @web.post('/')
    async create(req: Request, res: Response): Promise<void> {
        req.check(CreateServerSchema)

        if (req.user.servers.count() >= config.limits.user.servers) {
            throw new HTTPError('MAXIMUM_SERVERS')
        }

        const server = Server.from({
            ...req.body,
            owner: req.user
        })

        const chat = Channel.from({
            type: ChannelTypes.TEXT,
            server_id: server._id,
            name: 'general'
        })

        const category = Channel.from({
            type: ChannelTypes.CATEGORY,
            name: 'General',
            server_id: server._id,
            channels: [chat._id]
        })

        const member = Member.from({
            _id: req.user._id,
            server: server
        })

        const user = req.user

        user.servers.add(server)

        await db.save([
            server,
            chat,
            category,
            member,
            user
        ])

        res.json(server)
    }
}