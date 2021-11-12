import { Base } from './Base'
import { validator } from '../utils'
import db from '../database'
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
    embeds: Embed[] = []
    attachments: Attachment[] = []
    content?: string    
    mentions: ID[] = []
    replies: Reply[] = []
    channel_id!: ID    
    author_id!: ID

    isEmpty(): boolean {
        return !this.content?.length && !this.attachments.length
    }

    static from(opts: CreateMessageOptions): Message {
        return Object.assign(opts, new Message())
    }

    static toSQL(): string {
        return `CREATE TABLE messages IF NOT EXISTS (
            id BIGINT NOT NULL,
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