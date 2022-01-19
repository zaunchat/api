import { Base } from '.'
import { validator } from '../utils'
import { getaway } from '../getaway'
import sql from '../database'
import config from '../config'

export interface CreateMessageOptions extends Options<Message> {
  author_id: string
  channel_id: string
}

export const CreateMessageSchema = validator.compile({
  content: `string|max:${config.limits.message.length}|min:1`,
  attachments: {
    type: 'array',
    items: {
      $$type: "object",
      id: 'snowflake',
      name: 'string'
    },
    max: config.limits.message.attachments
  },
  replies: {
    type: 'array',
    items: {
      $$type: "object",
      id: 'snowflake',
      mention: 'boolean'
    },
    max: config.limits.message.replies,
    unique: true
  }
})

export const UpdateMessageSchema = validator.compile({
  content: `string|max:${config.limits.message.length}|min:1`
})


export interface Embed {
  title: string
  description: string
  footer: string
}

export interface Attachment {
  id: string
  name: string
}

export interface Reply {
  id: string
  mention: boolean
}

export class Message extends Base {
  created_at = Date.now()
  edited_at: Nullable<number> = null
  content: Nullable<string> = null
  embeds: Embed[] = []
  attachments: Attachment[] = []
  mentions: string[] = []
  replies: Reply[] = []
  channel_id!: string
  author_id!: string

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

  static from(opts: CreateMessageOptions): Message {
    return Object.assign(new Message(), opts)
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            created_at TIMESTAMP DEFAULT current_timestamp,
            edited_at TIMESTAMP,
            content VARCHAR(${config.limits.message.length}),
            embeds JSONB NOT NULL,
            attachments JSON NOT NULL,
            replies JSONB NOT NULL,
            channel_id BIGINT NOT NULL,
            author_id BIGINT NOT NULL,
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
            FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
        )`)
  }
}
