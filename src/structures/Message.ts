import { Base } from './Base'
import { Property, Entity, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import db from '../database'

export interface CreateMessageOptions extends Partial<Message> {
    authorId: string
    channelId: string
}

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
    channelId!: string

    @Property()
    authorId!: string

    static from(options: CreateMessageOptions): Message {
        return wrap(new Message().setID()).assign(options)
    }

    static find(query: FilterQuery<Message>, options?: FindOptions<Message>): Promise<Message[]> {
        return db.get(Message).find(query, options)
    }

    static findOne(query: FilterQuery<Message>): Promise<Message | null> {
        return db.get(Message).findOne(query)
    }

    async save(options?: Partial<Message>): Promise<void> {
        await db.get(Message).persistAndFlush(options ? wrap(this).assign(options) : this)
    }
}