import { Base } from './Base'
import { nanoid } from 'nanoid'
import { HTTPError } from '../errors'
import sql from '../database'

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

  static from(opts: CreateSessionOptions): Session {
    return Object.assign(new Session(), opts)
  }

  static async find(where: string, select: (keyof Session | '*')[] = ['*'], limit = 100): Promise<Session[]> {
    const result: Session[] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where} LIMIT ${limit}`)
    return result.map((row) => Session.from(row))
  }

  static async findOne(where: string, select: (keyof Session | '*')[] = ['*']): Promise<Session> {
    const [session]: [Session?] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where}`)

    if (session) return Session.from(session)

    throw new HTTPError('UNKNOWN_SESSION')
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            token VARCHAR(64) NOT NULL,
            user_id BIGINT NOT NULL,
            info JSON,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )`)
  }
}
