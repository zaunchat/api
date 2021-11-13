import { Base, Session, Server, Member } from '.'
import { validator } from '../utils'
import sql from '../database'
import config from '../config'


export enum RelationshipStatus {
    FRIEND,
    OUTGOING,
    IN_COMING,
    BLOCKED,
    BLOCKED_OTHER
}


export interface CreateUserOptions extends Partial<User> {
    username: string
    password: string
    email: string
}

export const CreateUserSchema = validator.compile({
    username: {
        type: 'string',
        min: 3,
        max: config.limits.user.username
    },
    email: {
        type: 'string',
        min: 3,
        max: 320
    },
    password: {
        type: 'string',
        min: 8,
        max: 72
    }
})

export const LoginUserSchema = validator.compile({
    email: { type: 'string', min: 3, max: 320 },
    password: { type: 'string', min: 8, max: 72 }
})

export const LogoutUserSchema = validator.compile({
    token: { type: 'string' },
    user_id: { type: 'string' }
})

export interface Presence {
    text?: string
    status: PresenceStatus
}

export enum PresenceStatus {
    ONLINE,
    OFFLINE,
    IDLE,
    DND
} Member


export class User extends Base {
    username!: string
    password!: string
    email!: string
    presence = { status: PresenceStatus.OFFLINE } as Presence
    badges = 0
    avatar?: string

    async fetchServers(): Promise<ID[]> {
        const res = await sql<Member[]>`SELECT * FROM members WHERE id = ${this.id} RETURNING server_id`
        return res.map((m) => m.server_id)
    }

    async fetchSessions(): Promise<Session[]> {
        const res = await sql<Session[]>`SELECT * FROM sessions WHERE user_id = ${this.id}`
        return res
    }

    // TODO:
    //  async fetchRelations(): Promise<unknown[]> {}

    static async fetchOne(id: ID): Promise<User> {
        const res = await sql<User[]>`SELECT * FROM users WHERE id = ${id}`
        return User.from(res[0])
    }

    static async fetchByToken(token: string): Promise<User | null> {
        const res = await sql<User[]>`SELECT * FROM users 
         LEFT JOIN sessions
         ON sessions.user_id = users.id
         WHERE verified = TRUE AND sessions.token = ${token}`
         
        if (!res.length) return null

        return User.from(res[0])
    }

    static from(opts: CreateUserOptions): User {
        return Object.assign(opts, new User())
    }

    static toSQL() {
        return `CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY,
            username VARCHAR(${config.limits.user.username}) NOT NULL,
            password VARCHAR(32) NOT NULL,
            email VARCHAR(255) NOT NULL,
            avatar VARCHAR(64),
            badges INTEGER DEFAULT 0,
            presence JSON NOT NULL,
            verified BOOLEAN DEFAULT FALSE
        )`
    }
}
