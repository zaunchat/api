import { Base, Role, Channel, Member } from '.'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
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
    description?: string    
    icon?: string
    banner?: string
    owner_id!: ID
    permissions = DEFAULT_PERMISSION_EVERYONE

    async fetchMembers(): Promise<Member[]> {
        const res = await sql<Member[]>`SELECT * FROM members WHERE server_id = ${this.id}`
        return res
    }

    async fetchRoles(): Promise<Role[]> {
        const res = await sql<Role[]>`SELECT * FROM roles WHERE server_id = ${this.id}`
        return res
    }

    async fetchChannels(): Promise<Channel[]> {
        const res = await sql<Channel[]>`SELECT * FROM channels WHERE server_id = ${this.id}`
        return res
    }

    static from(opts: CreateServerOptions): Server {
        return Object.assign(opts, new Server())
    }


    static toSQL(): string {
        return `CREATE TABLE IF NOT EXISTS servers (
            id BIGINT PRIMARY KEY,
            name VARCHAR(${config.limits.server.name}) NOT NULL,
            description VARCHAR(${config.limits.server.description}),
            icon VARCHAR(64),
            banner VARCHAR(64),
            owner_id BIGINT NOT NULL,
            permissions BIGINT NOT NULL,
            FOREIGN KEY (owner_id) REFERENCES users(id)
        )`
    }
}