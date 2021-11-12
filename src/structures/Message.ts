import { Base, User, Channel } from '.'
import { validator } from '../utils'
import db from '../database'
import config from '../config'

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


export class Message extends Base {
    created_at = Date.now()
    edited_at?: number
    embeds: unknown[] = []
    attachments: unknown[] = []
    content?: string    
    mentions: ID[] = []
    replies: { id: ID, mention: boolean }[] = []
    channel_id!: ID    
    author_id!: ID

    isEmpty(): boolean {
        return !this.content?.length && !this.attachments.length
    }

    static toSQL(): string {
        return `CREATE TABLE messages IF NOT EXISTS (
            id BIGINT NOT NULL,
            created_at TIMESTAMP DEFAULT current_timestamp,
            edited_at TIMESTAMP,
            content TEXT,
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