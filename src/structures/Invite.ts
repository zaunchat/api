import { Base } from './Base'
import { nanoid } from 'nanoid'
import sql from '../database'


export interface CreateInviteOptions extends Partial<Invite> {
	inviter_id: ID
	channel_id: ID
}


export class Invite extends Base {
	code = nanoid(8)
	uses = 0
	inviter_id!: ID
	channel_id!: ID

	static from(opts: CreateInviteOptions): Invite {
        return Object.assign(opts, new Invite())
    }

	static toSQL(): string {
		return `CREATE TABLE IF NOT EXISTS invites (
			id BIGINT PRIMARY KEY,
			code VARCHAR(8) NOT NULL,
			uses INTEGER DEFAULT 0,
			inviter_id BIGINT NOT NULL,
			channel_id BIGINT NOT NULL,
			FOREIGN KEY (inviter_id) REFERENCES users(id),
			FOREIGN KEY (channel_id) REFERENCES channels(id),
		)`
	}
}