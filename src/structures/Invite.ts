import { Entity, Property, PrimaryKey, OneToOne, FindOptions, FilterQuery, wrap } from '@mikro-orm/core'
import { nanoid } from 'nanoid'
import { User, Channel } from '.'
import db from '../database'


export interface CreateInviteOptions extends Partial<Invite> {
	inviter: User
	channel: Channel
}

@Entity({ tableName: 'invites' })
export class Invite {
	@PrimaryKey({ onCreate: () => nanoid(8), unique: true })
	code!: string

	@Property()
	uses = 0

	@OneToOne('User')
	inviter!: User

	@OneToOne('Channel')
	channel!: Channel

	static from(options: CreateInviteOptions): Invite {
		return wrap(new Invite()).assign(options)
	}

	static find(query: FilterQuery<Invite>, options?: FindOptions<Invite>): Promise<Invite[]> {
		return db.get(Invite).find(query, options)
	}

	static findOne(query: FilterQuery<Invite>): Promise<Invite | null> {
		return db.get(Invite).findOne(query)
	}

	static async save(...invites: Invite[]): Promise<void> {
		await db.get(Invite).persistAndFlush(invites)
	}

	async save(options?: Partial<Invite>): Promise<this> {
		await Invite.save(options ? wrap(this).assign(options) : this)
		return this
	}

	async delete(): Promise<void> {
		await db.get(Invite).removeAndFlush(this)
	}
}