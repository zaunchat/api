import * as web from 'express-decorators'
import { Response, Request, NextFunction } from '@tinyhttp/app'
import { Role, CreateRoleSchema, Member } from '../../structures'
import { Permissions } from '../../utils'
import config from '../../config'


@web.basePath('/servers/:server_id/roles')
export class ServerRoleController {
  @web.use()
  async authentication(req: Request, _res: Response, next: NextFunction): Promise<void> {
    const exists = await Member.findOne({
      id: req.user.id,
      server_id: req.params.server_id
    }).catch(() => null)

    if (!exists) {
      req.throw('UNKNOWN_SERVER')
    }

    next()
  }

  @web.get('/')
  async fetchMany(req: Request, res: Response): Promise<void> {
    const roles = await Role.find({ server_id: req.params.server_id })
    res.json(roles)
  }

  @web.get('/:role_id')
  async fetchOne(req: Request, res: Response): Promise<void> {
    const { server_id, role_id } = req.params
    const role = await Role.findOne({ id: role_id, server_id })
    res.json(role)
  }

  @web.post('/')
  async create(req: Request, res: Response): Promise<void> {
    req.check(CreateRoleSchema)

    const roleCount = await Role.count(`server_id = ${req.params.server_id}`)

    if (roleCount >= config.limits.server.roles) {
      req.throw('MAXIMUM_ROLES')
    }

    const permissions = await Permissions.fetch({
      user: req.user,
      server: req.params.server_id
    })

    if (!permissions.has(Permissions.FLAGS.MANAGE_ROLES)) {
      req.throw('MISSING_PERMISSIONS')
    }

    const role = await Role.from({
      name: 'new role',
      server_id: req.params.server_id
    }).save()

    res.json(role)
  }
}
