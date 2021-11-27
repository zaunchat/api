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
  user_id!: ID
  info!: DeviceInfo

  static from(opts: CreateSessionOptions): Session {
    return Object.assign(new Session(), opts)
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            token VARCHAR(64) NOT NULL,
            user_id BIGINT NOT NULL,
            info JSONB,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )`)
  }
}
