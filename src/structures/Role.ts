import { Base } from './Base'
import { validator } from '../utils'
import db from '../database'


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

    static from(opts: CreateRoleOptions): Role {
        return Object.assign(opts, new Role())
    }
    
    static toSQL(): string {
        return `CREATE TABLE roles IF NOT EXISTS (
            id BIGINT NOT NULL,
            name VARCHAR(32) NOT NULL,
            permissions BIGINT NOT NULL DEFAULT 0,
            hoist BOOLEAN NOT NULL DEFAULT FALSE,
            server_id BIGINT NOT NULL,
            FOREIGN KEY (server_id) REFERENCES servers(id)
        )`
    }
}