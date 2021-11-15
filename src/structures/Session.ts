import { Base } from './Base'
import { nanoid } from 'nanoid'
import sql from '../database'
import { HTTPError } from '../errors'

export interface CreateSessionOptions extends Partial<Session> {
    user_id: ID
}

interface DeviceInfo {
    name?: string
}

export class Session extends Base {
    token = nanoid(64)
    user_id!: ID
    info!: DeviceInfo

    static find: (statement: string, select?: (keyof Session)[], limit?: number) => Promise<Session[]>
    static from: (opts: CreateSessionOptions) => Session

    static async findOne(statement: string, select?: (keyof Session)[]): Promise<Session> {
        const result = await super.findOne(statement, select)

        if (result) return result as Session

        throw new HTTPError('UNKNOWN_SESSION')
    }

    static async init(): Promise<void> {
        await sql`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            token VARCHAR(64) NOT NULL,
            user_id BIGINT NOT NULL,
            info JSON,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )`
    }
}