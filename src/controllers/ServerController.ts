import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Category, Member, Role, Server, TextChannel } from '../structures'
import { HTTPError } from '../errors'
import { getaway } from '../server'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
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

        const generalChat = TextChannel.from({
            name: 'general',
            serverId: server._id
        })

        const category = Category.from({
            name: 'General',
            serverId: server._id,
            channels: [generalChat._id]
        })


        await Promise.all([
            server.save(),
            Member.from({ _id: req.user._id, serverId: server._id }).save(),
            generalChat.save(),
            category.save()
        ])

        getaway.emit('SERVER_CREATE', server)

        res.json(server)
    }
}