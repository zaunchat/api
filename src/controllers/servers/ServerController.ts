import * as web from 'express-decorators'
import { Response, Request } from '@tinyhttp/app'
import { Server, Channel, CreateServerSchema, Member, ChannelTypes } from '../../structures'
import config from '../../config'

@web.basePath('/servers')
export class ServerController {
  @web.get('/')
  async fetchMany(req: Request, res: Response): Promise<void> {
    res.json(await req.user.fetchServers())
  }

  @web.get('/:server_id')
  async fetchOne(req: Request, res: Response): Promise<void> {
    const server = await Server.findOne({ id: req.params.server_id })
    res.json(server)
  }

  @web.route('delete', '/:server_id')
  async delete(req: Request, res: Response): Promise<void> {
    const server = await Server.findOne({ id: req.params.server_id })

    if (req.user.id !== server.owner_id) {
      req.throw('MISSING_ACCESS')
    }

    await server.delete()

    res.sendStatus(202)
  }


  @web.post('/')
  async create(req: Request, res: Response): Promise<void> {
    req.check(CreateServerSchema)

    const serverCount = await Member.count(`id = ${req.user.id}`)

    if (serverCount >= config.limits.user.servers) {
      req.throw('MAXIMUM_SERVERS')
    }

    const server = Server.from({
      ...req.body,
      owner_id: req.user.id
    })

    const category = Channel.from({
      type: ChannelTypes.CATEGORY,
      server_id: server.id,
      name: 'General'
    })

    const chat = Channel.from({
      type: ChannelTypes.TEXT,
      server_id: server.id,
      name: 'general',
      parents: [category.id]
    })

    const member = Member.from({
      id: req.user.id,
      server_id: server.id
    })

    await server.save()
    await chat.save()
    await category.save()
    await member.save()

    res.json(server)
  }
}
