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
        max: 30
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
    color?: number
    hoist = false
    server_id!: ID

	static find: (statement: string, select?: (keyof Role)[], limit?: number) => Promise<Role[]>
    static from: (opts: CreateRoleOptions) => Role
    static async findOne(statement: string, select?: (keyof Role)[]): Promise<Role> {
        const result = await super.findOne(statement, select)

        if (result) return result as Role

        throw new HTTPError('UNKNOWN_ROLE')
    }
    
    static async init(): Promise<void> {

        await sql`CREATE TABLE IF NOT EXISTS ${sql(this.tableName)} (
            id BIGINT PRIMARY KEY,
            name VARCHAR(32) NOT NULL,
            permissions BIGINT NOT NULL DEFAULT 0,
            hoist BOOLEAN NOT NULL DEFAULT FALSE,
            server_id BIGINT NOT NULL,
            FOREIGN KEY (server_id) REFERENCES servers(id)
        )`
    }
}