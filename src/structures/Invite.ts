import { Base } from './Base'
import { nanoid } from 'nanoid'
import { HTTPError } from '../errors'
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

  static async find(where: string, select: (keyof Invite | '*')[] = ['*'], limit = 100): Promise<Invite[]> {
    const result: Invite[] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where} LIMIT ${limit}`)
    return result.map((row) => Invite.from(row))
  }

  static async findOne(where: string, select: (keyof Invite | '*')[] = ['*']): Promise<Invite> {
    const [invite]: [Invite?] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where}`)

    if (invite) return Invite.from(invite)

    throw new HTTPError('UNKNOWN_INVITE')
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
			id BIGINT PRIMARY KEY,
			code VARCHAR(8) NOT NULL,
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
