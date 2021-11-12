import { Base, Session, Server } from '.'
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
    userid: { type: 'string' }
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
}


export class User extends Base {
    username!: string
    password!: string
    email!: string
    presence = { status: PresenceStatus.OFFLINE } as Presence
    badges = 0
    avatar?: string
    async fetchServers(): Promise<Server[]> {
        sql`SELECT * FROM SERVERS `
        return []
    }

    async fetchSessions(): Promise<Session[]> {
        return []
    }

    async fetchRelations(): Promise<unknown[]> {
        return []
    }

    static async fetchOne(id: ID): Promise<User> {
       const res = await sql<User[]>`SELECT * FROM users WHERE id = ${id}`
       return res[0]
    }

    static from(opts: CreateUserOptions): User {
        return Object.assign(opts, new User())
    }

    static toSQL() {
        return `CREATE TABLE users IF NOT EXISTS (
            id BIGINT NOT NULL,
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
