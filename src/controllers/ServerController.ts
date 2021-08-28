import { Response, Request } from '@tinyhttp/app'
import * as web from 'express-decorators'
import { Category, Member, Role, Server, TextChannel } from '../structures'
import db from '../database'
import { HTTPError } from '../errors'
import { getaway } from '../server'
import { DEFAULT_PERMISSION_SERVER } from '../utils'
import Validator from 'fastest-validator'

const validator = new Validator()


@web.basePath('/servers')
export class ServerController {
    checks = {
        createServer: validator.compile({
            name: { type: 'string' }
        })
    }

    @web.post('/:serverId')
    async fetchServer(req: Request, res: Response): Promise<void> {
        const server = await db.get(Server).findOne({
            _id: req.params.serverId,
            deleted: false
        })

        if (!server) {
            return void res.status(404).send(new HTTPError('UNKNOWN_SERVER'))
        }

        const isExistsInServer = await db.get(Member).findOne({
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

        const server = Server.from({
            ...req.body,
            ownerId: req.user._id
        })

        const defaultRole = Role.from({
            _id: server._id,
            name: 'everyone',
            permissions: DEFAULT_PERMISSION_SERVER
        })

        server.roles.push(defaultRole)

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
            db.get(Server).persistAndFlush(server),
            db.get(Member).persistAndFlush(Member.from({
                _id: req.user._id,
                serverId: server._id,
                roles: [server._id]
            })),
            db.get(TextChannel).persistAndFlush(generalChat),
            db.get(Category).persistAndFlush(category)
        ])

        getaway.emit('SERVER_CREATE', server)

        res.json(server)
    }
}