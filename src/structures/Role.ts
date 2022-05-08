import { Base } from '.'
import { validator } from '../utils'
import sql from '../database'

export interface CreateRoleOptions extends Options<Role> {
  name: string
  server_id: string
}

export const CreateRoleSchema = validator.compile({
  name: 'string|min:1|max:32',
  color: 'number|optional',
  permissions: 'number|optional',
  hoist: 'boolean|optional'
})

export class Role extends Base {
  name!: string
  permissions = 0n
  color = 0
  hoist = false
  server_id!: string

  static from(opts: CreateRoleOptions): Role {
    return Object.assign(new Role(), opts)
  }
}
