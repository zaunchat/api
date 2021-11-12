
import { nanoid } from 'nanoid'
import { User, Channel } from '.'
import db from '../database'


export interface CreateInviteOptions extends Partial<Invite> {
	inviter: User
	channel: Channel
}


export class Invite {
	code = nanoid(8)
	uses = 0
	inviter_id!: ID
	channel_id!: ID
	static toSQL(): string {
		return `CREATE TABLE invites IF NOT EXISTS (
			id BIGINT NOT NULL,
			code VARCHAR(8) NOT NULL,
			uses INTEGER DEFAULT 0,
			inviter_id BIGINT NOT NULL,
			channel_id BIGINT NOT NULL,
			FOREIGN KEY (inviter_id) REFERENCES users(id),
			FOREIGN KEY (channel_id) REFERENCES channels(id),
		)`
	}
}