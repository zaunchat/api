import { Base } from './Base'
import { validator } from '../utils'
import sql from '../database'

export interface CreateRoleOptions extends Partial<Role> {
  name: string
  server_id: ID
}

export const CreateRoleSchema = validator.compile({
  name: {
    type: 'string',
    min: 1,
    max: 32
  },
  color: {
    type: 'number',
    optional: true
  },
  permissions: {
    type: 'number',
    optional: true
  },
  hoist: {
    type: 'boolean',
    optional: true
  }
})


export class Role extends Base {
  name!: string
  permissions = 0
  color = 0
  hoist = false
  server_id!: ID

  static from(opts: CreateRoleOptions): Role {
    return Object.assign(new Role(), opts)
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            name VARCHAR(32) NOT NULL,
            permissions BIGINT NOT NULL DEFAULT 0,
            hoist BOOLEAN NOT NULL DEFAULT FALSE,
            server_id BIGINT NOT NULL,
            FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
        )`)
  }
}
