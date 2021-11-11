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
        const server = req.user.servers.getItems().find((s) => {
            return s._id === req.params.server_id
        })

        if (!server) {
            throw new HTTPError('UNKNOWN_SERVER')
        }

        res.json(server)
    }

    @web.route('delete', '/:server_id')
    async delete(req: Request, res: Response): Promise<void> {
        const server = req.user.servers.getItems().find((s) => {
            return s._id === req.params.server_id
        })

        if (!server) {
            throw new HTTPError('UNKNOWN_SERVER')
        }

        if (req.user._id !== server.owner._id) {
            throw new HTTPError('MISSING_ACCESS')
        }

        await server.delete()

        res.sendStatus(202)
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
            server,
            name: 'general'
        })

        const category = Channel.from({
            type: ChannelTypes.CATEGORY,
            name: 'General',
            server,
            channels: [chat._id]
        })

        const member = Member.from({
            _id: req.user._id,
            server
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