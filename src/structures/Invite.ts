import { Base } from './Base'
import { nanoid } from 'nanoid'
import sql from '../database'
import { HTTPError } from '../errors'

export interface CreateInviteOptions extends Partial<Invite> {
	inviter_id: ID
	channel_id: ID
}


export class Invite extends Base {
	code = nanoid(8)
	uses = 0
	inviter_id!: ID
	channel_id!: ID
	server_id!: ID

	static find: (statement: string, select?: (keyof Invite)[], limit?: number) => Promise<Invite[]>
	static from: (opts: CreateInviteOptions) => Invite

	static async findOne(statement: string, select?: (keyof Invite)[]): Promise<Invite> {
        const result = await super.findOne(statement, select)

        if (result) return result as Invite

        throw new HTTPError('UNKNOWN_INVITE')
    }

	static async init(): Promise<void> {
		await sql`CREATE TABLE IF NOT EXISTS ${this.tableName} (
			id BIGINT PRIMARY KEY,
			code VARCHAR(8) NOT NULL,
			uses INTEGER DEFAULT 0,
			inviter_id BIGINT NOT NULL,
			channel_id BIGINT NOT NULL,
			server_id BIGINT NOT NULL,
			FOREIGN KEY (inviter_id) REFERENCES users(id),
			FOREIGN KEY (channel_id) REFERENCES channels(id),
			FOREIGN KEY (server_id) REFERENCES servers(id),
		)`
	}
}