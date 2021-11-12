import { Base, Presence, Session, Server } from '.'
import { validator } from '../utils'
import db from '../database'
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

export const PUBLIC_USER_ITEMS: (keyof User)[] = [
    'id',
    'username',
    'avatar',
    'badges'
]


export class User extends Base {
    username!: string
    password!: string
    email!: string
    presence = Presence.from({})
    badges = 0
    avatar?: string
    verified = false
    
    static toSQL() {
        return `CREATE TABLE users IF NOT EXISTS (
            id BIGINT NOT NULL DEFAULT id_generator(),
            username VARCHAR(${config.limits.user.username}) NOT NULL,
            password VARCHAR(32) NOT NULL,
            email VARCHAR(255) NOT NULL,
            relations,
            servers,
            avatar VARCHAR(32),
            badges INTEGER DEFAULT 0,
            sessions,
            verified BOOLEAN DEFAULT FALSE
        )`
    }
}
