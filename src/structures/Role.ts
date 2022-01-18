import { Base } from '.'
import { validator } from '@utils'
import sql from '@database'

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

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            name VARCHAR(32) NOT NULL,
            permissions BIGINT NOT NULL,
            hoist BOOLEAN NOT NULL,
            server_id BIGINT NOT NULL,
            FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
        )`)
  }
}
