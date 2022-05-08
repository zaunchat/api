import { Base, Role, Channel, Member, ServerChannel } from '.'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
import sql from '../database'
import config from '../config'


export interface CreateServerOptions extends Options<Server> {
  name: string
  owner_id: string
}

export const CreateServerSchema = validator.compile({
  name: `string|min:1|max:${config.limits.server.name}`
})

export const UpdateServerSchema = validator.compile({
  name: `string|min:1|max:${config.limits.server.name}|optional`,
  description: `string|min:0|max:${config.limits.server.description}|optional`
})

export class Server extends Base {
  name!: string
  description: Nullable<string> = null
  icon: Nullable<string> = null
  banner: Nullable<string> = null
  owner_id!: string
  permissions = DEFAULT_PERMISSION_EVERYONE

  static from(opts: CreateServerOptions): Server {
    return Object.assign(new Server(), opts)
  }

  fetchMembers(): Promise<Member[]> {
    return Member.find({ server_id: this.id })
  }

  fetchRoles(): Promise<Role[]> {
    return Role.find({ server_id: this.id })
  }

  fetchChannels(): Promise<ServerChannel[]> {
    return Channel.find<ServerChannel>({ server_id: this.id })
  }
}
