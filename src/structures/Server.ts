import { Base, Role, User, Channel } from '.'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
import db from '../database'
import config from '../config'

export interface CreateServerOptions extends Partial<Server> {
    name: string
    owner: User
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
    description?: string    
    icon?: string
    banner?: string
    owner!: User
    // roles = new Collection<Role>(this)
    // channels = new Collection<Channel>(this)
    permissions = DEFAULT_PERMISSION_EVERYONE
}