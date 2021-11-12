import { Base } from './Base'
import { nanoid } from 'nanoid'
import sql from '../database'

export interface CreateSessionOptions extends Partial<Session> {
    user_id: ID
}

interface DeviceInfo {
    name?: string
}

export class Session extends Base {
    token = nanoid(64)
    user_id!: string
    info!: DeviceInfo

    static from(opts: CreateSessionOptions): Session {
        return Object.assign(opts, new Session())
    }

    static async fetchOneByToken(token: string): Promise<Session> {
        const res = await sql<Session[]>`SELECT * FROM sessions WHERE token = ${token}`
        return res[0]
    }

    static toSQL(): string {
        return `CREATE TABLE sessions IF NOT EXISTS (
            id BIGINT NOT NULL,
            token VARCHAR(64) NOT NULL,
            user_id BIGINT NOT NULL,
            info JSON,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )`
    }
}