import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Server, Member, Category, TextChannel } from '../structures'
import { HTTPError } from '../errors'
import { getaway } from '../server'
import { validator } from '../utils'
import config from '../../config'


@web.basePath('/servers')
export class ServerController {
    checks = {
        createServer: validator.compile({
            name: { type: 'string' }
        })
    }

    @web.post('/:serverId')
    async fetchServer(req: Request, res: Response): Promise<void> {
        const server = await Server.findOne({
            _id: req.params.serverId,
            deleted: false
        })

        if (!server) {
            return void res.status(404).send(new HTTPError('UNKNOWN_SERVER'))
        }

        const isExistsInServer = await Member.findOne({
            serverId: server._id,
            _id: req.user._id
        })

        if (!isExistsInServer) {
            return void res.status(404).send(new HTTPError('MISSING_ACCESS'))
        }

        res.json(server)
    }

    @web.post('/')
    async createServer(req: Request, res: Response): Promise<void> {
        const valid = this.checks.createServer(req.body)

        if (valid !== true) {
            return void res.status(400).send(valid)
        }

        if (req.user.servers.length >= config.max.user.servers) {
            return void res.status(403).send(new HTTPError('MAXIMUM_SERVERS'))
        }

        const server = Server.from({
            ...req.body,
            ownerId: req.user._id
        })

        const chat = TextChannel.from({
            name: 'general',
            serverId: server._id
        })

        await Promise.all([
            server.save(),
            chat.save(),
            Category.from({
                name: 'General',
                serverId: server._id,
                channels: [chat._id]
            }).save(),
            Member.from({
                _id: req.user._id,
                serverId: server._id
            }).save(),
        ])

        getaway.emit('SERVER_CREATE', server)

        res.json(server)
    }
}