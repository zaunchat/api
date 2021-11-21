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
  edited_at: number | null = null
  content: string | null = null
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

  static from(opts: CreateMessageOptions): Message {
    return Object.assign(new Message(), opts)
  }

  static async find(where: string, select: (keyof Message | '*')[] = ['*'], limit = 100): Promise<Message[]> {
    const result: Message[] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where} LIMIT ${limit}`)
    return result.map((row) => Message.from(row))
  }

  static async findOne(where: string, select: (keyof Message | '*')[] = ['*']): Promise<Message> {
    const [message]: [Message?] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where}`)

    if (message) return Message.from(message)

    throw new HTTPError('UNKNOWN_USER')
  }


  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            created_at TIMESTAMP DEFAULT current_timestamp,
            edited_at TIMESTAMP,
            content VARCHAR(${config.limits.message.length}),
            embeds JSON NOT NULL,
            attachments JSON NOT NULL,
            replies JSON NOT NULL,
            channel_id BIGINT NOT NULL,
            author_id BIGINT NOT NULL,
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
            FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
        )`)
  }
}
