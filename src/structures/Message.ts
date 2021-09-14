import { Base, User, Channel } from '.'
import { Property, Entity, wrap, FilterQuery, FindOptions, OneToOne } from '@mikro-orm/core'
import { validator } from '../utils'
import db from '../database'
import config from '../../config'

export interface CreateMessageOptions extends Partial<Message> {
    author: User
    channel: Channel
}

export const CreateMessageSchema = validator.compile({
    content: {
        type: 'string',
        min: 1,
        max: config.limits.message.length
    },
    $$strict: true
})

@Entity({ tableName: 'messages' })
export class Message extends Base {
    @Property()
    created_timestamp: number = Date.now()

    @Property({ onUpdate: () => Date.now(), nullable: true })
    edited_timestamp?: number

    @Property()
    embeds: unknown[] = []

    @Property()
    attachments: unknown[] = []

    @Property({ nullable: true })
    content?: string

    @Property()
    mentions: ID[] = []

    @Property()
    replies: {
        id: ID
        mention: boolean
    }[] = []

    @OneToOne({ entity: () => Channel })
    channel!: Channel

    @OneToOne({ entity: () => User })
    author!: User

    isEmpty(): boolean {
        return !this.content?.length && !this.attachments.length
    }

    static from(options: CreateMessageOptions): Message {
        return wrap(new Message().setID()).assign(options)
    }

    static find(query: FilterQuery<Message>, options?: FindOptions<Message>): Promise<Message[]> {
        return db.get(Message).find(query, options)
    }

    static findOne(query: FilterQuery<Message>): Promise<Message | null> {
        return db.get(Message).findOne(query)
    }

    static async save(...messages: Message[]): Promise<void> {
        await db.get(Message).persistAndFlush(messages)
    }

    async save(options?: Partial<Message>): Promise<this> {
        await Message.save(options ? wrap(this).assign(options) : this)
        return this
    }

    async delete(): Promise<void> {
        await db.get(Message).removeAndFlush(this)
    }
}