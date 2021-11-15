import { Base, Role, Channel, Member } from '.'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
import sql from '../database'
import config from '../config'
import { HTTPError } from '../errors'

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

    static find: (statement: string, select?: (keyof Server)[], limit?: number) => Promise<Server[]>
    static from: (opts: CreateServerOptions) => Server
    static async findOne(statement: string, select?: (keyof Server)[]): Promise<Server> {
        const result = await super.findOne(statement, select)

        if (result) return result as Server

        throw new HTTPError('UNKNOWN_SERVER')
    }


    fetchMembers(): Promise<Member[]> {
        return sql<Member[]>`SELECT * FROM members WHERE server_id = ${this.id}`.then((m) => m.map(Member.from))
    }

    fetchRoles(): Promise<Role[]> {
        return sql<Role[]>`SELECT * FROM roles WHERE server_id = ${this.id}`.then((r) => r.map(Role.from))
    }

    fetchChannels(): Promise<Channel[]> {
        return sql<Channel[]>`SELECT * FROM channels WHERE server_id = ${this.id}`
    }

    static async init(): Promise<void> {
        await sql`CREATE TABLE IF NOT EXISTS ${this.tableName} (
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