import { HTTPError } from '../errors'
import { Request, Response, NextFunction } from '@tinyhttp/app'
import { Permissions, PermissionsResolvable } from '../utils'

export const permissions = (bits: PermissionsResolvable, type: 'has' | 'any' = 'has'): typeof middleware => {
  bits = Permissions.resolve(bits)

  const middleware = async (req: Request, _res: Response, next: NextFunction) => {
    const permissions = await Permissions.from(req)
    if (!permissions[type](bits)) throw new HTTPError('MISSING_PERMISSIONS')
    next()
  }

  return middleware
}


permissions.any = (bits: PermissionsResolvable) => permissions(bits, 'any')
permissions.has = (bits: PermissionsResolvable) => permissions(bits, 'has')
