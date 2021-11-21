import { Base } from './Base'
import { validator } from '../utils'
import { HTTPError } from '../errors'
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

  static async find(where: string, select: (keyof Role | '*')[] = ['*'], limit = 100): Promise<Role[]> {
    const result: Role[] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where} LIMIT ${limit}`)
    return result.map((row) => Role.from(row))
  }

  static async findOne(where: string, select: (keyof Role | '*')[] = ['*']): Promise<Role> {
    const [role]: [Role?] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where}`)

    if (role) return Role.from(role)

    throw new HTTPError('UNKNOWN_ROLE')
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
