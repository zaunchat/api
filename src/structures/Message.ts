import { Base } from './Base'
import { Property, Entity, wrap } from 'mikro-orm'


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
        return wrap(new Message()).assign(options)
    }
}