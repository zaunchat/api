import { Base } from '.'
import { nanoid } from 'nanoid'
import sql from '../database'

export interface CreateInviteOptions extends Options<Invite> {
  inviter_id: string
  channel_id: string
  server_id: string
}


export class Invite extends Base {
  readonly code = nanoid(8)
  uses = 0
  inviter_id!: string
  channel_id!: string
  server_id!: string

  static from(opts: CreateInviteOptions): Invite {
    return Object.assign(new Invite(), opts)
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
			id BIGINT PRIMARY KEY,
			code VARCHAR(8) NOT NULL UNIQUE,
			uses INTEGER DEFAULT 0,
			inviter_id BIGINT NOT NULL,
			channel_id BIGINT NOT NULL,
			server_id BIGINT NOT NULL,
			FOREIGN KEY (inviter_id) REFERENCES users(id),
			FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
			FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
		)`)
  }
}
