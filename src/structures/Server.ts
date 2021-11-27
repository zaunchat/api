import { Base, Role, Channel, Member } from '.'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
import { getaway } from '../getaway'
import sql from '../database'
import config from '../config'


export interface CreateServerOptions extends Partial<Server> {
  name: string
  owner_id: ID
}

export const CreateServerSchema = validator.compile({
  name: {
    type: 'string',
    min: 1,
    max: config.limits.server.name
  }
})

export const ModifyServerSchema = validator.compile({
  name: {
    type: 'string',
    min: 1,
    max: config.limits.server.name,
    optional: true
  },
  description: {
    type: 'string',
    min: 0,
    max: config.limits.server.description,
    optional: true
  }
})



export class Server extends Base {
  name!: string
  description: string | null = null
  icon: string | null = null
  banner: string | null = null
  owner_id!: ID
  permissions = DEFAULT_PERMISSION_EVERYONE

  static async onCreate(self: Server): Promise<void> {
    await getaway.subscribe(self.owner_id, self.id)
    await getaway.publish(self.id, 'SERVER_CREATE', self)
  }

  static async onUpdate(self: Server): Promise<void> {
    await getaway.publish(self.id, 'SERVER_UPDATE', self)
  }

  static async onDelete(self: Server): Promise<void> {
    await getaway.publish(self.id, 'SERVER_DELETE', { id: self.id })
  }

  static from(opts: CreateServerOptions): Server {
    return Object.assign(new Server(), opts)
  }

  fetchMembers(): Promise<Member[]> {
    return Member.find({ server_id: this.id })
  }

  fetchRoles(): Promise<Role[]> {
    return Role.find({ server_id: this.id })
  }

  fetchChannels(): Promise<Channel[]> {
    return Channel.find({ server_id: this.id })
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            name VARCHAR(${config.limits.server.name}) NOT NULL,
            description VARCHAR(${config.limits.server.description}),
            icon VARCHAR(64),
            banner VARCHAR(64),
            owner_id BIGINT NOT NULL,
            permissions BIGINT NOT NULL,
            FOREIGN KEY (owner_id) REFERENCES users(id)
        )`)
  }
}
