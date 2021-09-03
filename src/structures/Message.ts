import { Base } from './Base'
import { Property, Entity, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import { validator } from '../utils'
import db from '../database'
import config from '../../config'

export interface CreateMessageOptions extends Partial<Message> {
    authorId: Snowflake
    channelId: Snowflake
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
    @Property({ onCreate: () => Date.now() })
    createdTimestamp!: number

    @Property({ nullable: true, onUpdate: () => Date.now() })
    editedTimestamp?: number

    @Property()
    embeds: unknown[] = []

    @Property()
    attachments: unknown[] = []

    @Property({ nullable: true })
    content?: string

    @Property()
    mentions: string[] = []

    @Property()
    replies: string[] = []

    @Property()
    channelId!: Snowflake

    @Property()
    authorId!: Snowflake

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