import { Base, Role, Server } from '.'
import { validator } from '../utils'
import sql from '../database'
import config from '../config'

export interface CreateMemberOptions extends Partial<Member> {
    id: ID
    server_id: ID
}

export const CreateMemberSchema = validator.compile({
    nickname: {
        type: 'string',
        min: 0,
        max: config.limits.member.nickname,
        optional: true
    },
    roles: {
        type: 'array',
        items: 'string',
        optional: true
    }
})



export class Member extends Base {
    nickname?: string
    joined_timestamp = Date.now()
    server_id!: ID

    static from(opts: CreateMemberOptions): Member {
        return Object.assign(opts, new Member())
    }

    static toSQL(): string {
        return `CREATE TABLE IF NOT EXISTS members (
            id BIGINT PRIMARY KEY,
            joined_at TIMESTAMP DEFAULT current_timestamp,
            nickname VARCHAR(32),
            server_id BIGINT NOT NULL,
            FOREIGN KEY (server_id) REFERENCES servers(id)
            FOREIGN KEY (id) REFERENCES users(id)
        )`
    }
}