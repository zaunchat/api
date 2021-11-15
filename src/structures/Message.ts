import { Base } from './Base'
import { validator } from '../utils'
import { HTTPError } from '../errors'
import { getaway } from '../getaway'
import sql from '../database'
import config from '../config'

export interface CreateMessageOptions extends Partial<Message> {
    author_id: ID
    channel_id: ID
}

export const CreateMessageSchema = validator.compile({
    content: {
        type: 'string',
        min: 1,
        max: config.limits.message.length
    },
    $$strict: true
})


export interface Embed {
    title: string
    description: string
    footer: string
}

export interface Attachment {
    name: string
    id: string
}

export interface Reply {
    id: ID
    mention: boolean
}

export class Message extends Base {
    created_at = Date.now()
    edited_at?: number
    content?: string
    embeds: Embed[] = []
    attachments: Attachment[] = []
    mentions: ID[] = []
    replies: Reply[] = []
    channel_id!: ID
    author_id!: ID

    static async onCreate(self: Message): Promise<void> {
		await getaway.publish(self.channel_id, 'MESSAGE_CREATE', self)
    }

    static async onUpdate(self: Message): Promise<void> {
        await getaway.publish(self.channel_id, 'MESSAGE_UPDATE', self)
    }

    static async onDelete(self: Message): Promise<void> {
        await getaway.publish(self.channel_id, 'MESSAGE_DELETE', { id: self.id })
    }


    isEmpty(): boolean {
        return !this.content?.length && !this.attachments.length
    }

    static find: (statement: string, select?: (keyof Message)[], limit?: number) => Promise<Message[]>
    static from: (opts: CreateMessageOptions) => Message

    static async findOne(statement: string, select?: (keyof Message)[]): Promise<Message> {
        const result = await super.findOne(statement, select)

        if (result) return result as Message

        throw new HTTPError('UNKNOWN_MESSAGE')
    }

    static async init(): Promise<void> {
        await sql`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            created_at TIMESTAMP DEFAULT current_timestamp,
            edited_at TIMESTAMP,
            content VARCHAR(${config.limits.message.length}),
            embeds JSON NOT NULL,
            attachments JSON NOT NULL,
            replies JSON NOT NULL,
            channel_id BIGINT NOT NULL,
            author_id BIGINT NOT NULL,
            FOREIGN KEY (channel_id) REFERENCES channels(id)
            FOREIGN KEY (author_id) REFERENCES users(id)
        )`
    }
}