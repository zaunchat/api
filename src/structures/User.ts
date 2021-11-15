import { Base, Session, Member, Server } from '.'
import { validator } from '../utils'
import sql from '../database'
import config from '../config'
import { HTTPError } from '../errors'

export const PUBLIC_USER_PROPS = [
    'id',
    'username',
    'presence',
    'badges',
    'avatar'
]

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

export interface Relationship {
    id: ID,
    status: RelationshipStatus
}

export enum PresenceStatus {
    ONLINE,
    OFFLINE,
    IDLE,
    DND
}

export enum RelationshipStatus {
    FRIEND,
    OUTGOING,
    IN_COMING,
    BLOCKED,
    BLOCKED_OTHER
}


export class User extends Base {
    username!: string
    password!: string
    email!: string
    presence = { status: PresenceStatus.OFFLINE } as Presence
    relations: Relationship[] = []
    badges = 0
    avatar?: string
    verified = false

    static find: (statement: string, select?: (keyof User)[], limit?: number) => Promise<User[]>
    static from: (opts: CreateUserOptions) => User
    static async findOne(statement: string, select?: (keyof User)[]): Promise<User> {
        const result = await super.findOne(statement, select)

        if (result) return result as User

        throw new HTTPError('UNKNOWN_USER')
    }

    fetchServers(): Promise<Server[]> {
        return sql`SELECT * FROM servers WHERE id IN (
            SELECT server_id FROM members WHERE id = ${this.id}
        )`.then(res => res.map((m) => m.server_id))
    }

    fetchSessions(): Promise<Session[]> {
        return sql<Session[]>`SELECT * FROM sessions WHERE user_id = ${this.id}`.then(res => res.map(Session.from))
    }

    fetchRelations(): Promise<User[]> {
        return sql<User[]>`SELECT * FROM users WHERE id IN (${[...this.relations.keys()]})`.then(res => res.map(User.from))
    }

    static async fetchByToken(token: string): Promise<User | null> {
        const [user]: [User?] = await sql`SELECT * FROM users 
         LEFT JOIN sessions
         ON sessions.user_id = users.id
         WHERE verified = TRUE AND sessions.token = ${token}`

        if (!user) return null

        return User.from(user)
    }

    static async init(): Promise<void> {
        await sql`CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY,
            username VARCHAR(${config.limits.user.username}) NOT NULL,
            password VARCHAR(32) NOT NULL,
            email VARCHAR(255) NOT NULL,
            avatar VARCHAR(64),
            badges INTEGER DEFAULT 0,
            presence JSON NOT NULL,
            relations JSON NOT NULL,
            verified BOOLEAN DEFAULT FALSE
        )`
    }
}
