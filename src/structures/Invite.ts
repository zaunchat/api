import { Base } from './Base'
import { nanoid } from 'nanoid'
import sql from '../database'


export interface CreateInviteOptions extends Partial<Invite> {
  inviter_id: ID
  channel_id: ID
  server_id: ID
}


export class Invite extends Base {
  code = nanoid(8)
  uses = 0
  inviter_id!: ID
  channel_id!: ID
  server_id!: ID

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
