import { Base } from './Base'
import { DEFAULT_PERMISSION_DM, validator } from '../utils'
import { HTTPError } from '../errors'
import { getaway } from '../getaway'
import sql from '../database'
import config from '../config'

export enum ChannelTypes {
    DM,
    TEXT,
    VOICE,
    CATEGORY,
    GROUP,
    UNKNOWN = -1
}

export enum OverwriteType {
    ROLE,
    MEMBER
}

export interface ChannelOverwrite {
    id: ID
    type: OverwriteType
    allow: number
    deny: number
}

export type ChannelOverwrites = ChannelOverwrite[]

export interface DMChannel extends Channel {
    type: ChannelTypes.DM
}

export interface Group extends Channel {
    type: ChannelTypes.GROUP
    name: string
    owner_id: ID
    permissions: number
    recipients: ID[]
}

export interface TextChannel extends Channel {
    type: ChannelTypes.TEXT
    name: string
    server_id: ID
    overwrites: ChannelOverwrites
}

export interface Category extends Channel {
    type: ChannelTypes.CATEGORY
    name: string
    server_id: ID
    parents: ID[]
    overwrites: ChannelOverwrites
}


export const CreateGroupSchema = validator.compile({
    name: {
        type: 'string',
        min: 1,
        max: config.limits.group.name
    }
})


export const CreateTextChannelSchema = validator.compile({
    name: {
        type: 'string',
        min: 1,
        max: config.limits.channel.name
    },
    topic: {
        type: 'string',
        min: 1,
        max: config.limits.channel.topic,
        optional: true
    },
    $$strict: true
})

export const CreateCategorySchema = validator.compile({
    name: {
        type: 'string',
        min: 1,
        max: config.limits.channel.name
    }
})



export class Channel extends Base {
    type = ChannelTypes.UNKNOWN
    name?: string
    topic?: string
    server_id?: ID
    owner_id?: ID
    overwrites?: ChannelOverwrites
    icon?: string
    permissions = DEFAULT_PERMISSION_DM
    parents?: ID[]
    recipients?: ID[]


    static async onCreate(self: Channel): Promise<void> {
        if (self.recipients) {
            const recipients = self.recipients
            await Promise.all(recipients.map((id) => getaway.subscribe(id, [self.id])))
        }
        await getaway.publish(self.id, 'CHANNEL_CREATE', self)
    }

    static async onUpdate(self: Channel): Promise<void> {
        await getaway.publish(self.id, 'CHANNEL_UPDATE', self)
    }

    static async onDelete(self: Channel): Promise<void> {
        await getaway.publish(self.id, 'CHANNEL_DELETE', { id: self.id })
    }

    static from(opts: { type: ChannelTypes.TEXT } & Partial<TextChannel>): TextChannel
    static from(opts: { type: ChannelTypes.DM } & Partial<DMChannel>): DMChannel
    static from(opts: { type: ChannelTypes.CATEGORY } & Partial<Category>): Category
    static from(opts: { type: ChannelTypes.GROUP } & Partial<Group>): Group
    static from(opts: Partial<Channel>): Channel {
        return Object.assign(new Channel(), opts)
    }

    static find: (statement: string, select?: (keyof Channel)[], limit?: number) => Promise<Channel[]>

    static async findOne(statement: string, select?: (keyof Channel)[]): Promise<Channel> {
        const result = await super.findOne(statement, select)

        if (result) return result as Channel

        throw new HTTPError('UNKNOWN_CHANNEL')
    }


    static async init(): Promise<void> {
        await sql`CREATE TABLE IF NOT EXISTS ${sql(this.tableName)} (
            id BIGINT PRIMARY KEY,
            type INTEGER NOT NULL,
            name VARCHAR(${config.limits.channel.name}),
            topic VARCHAR(${config.limits.channel.topic}),
            permissions BIGINT DEFAULT 0,
            overwrites JSON,
            recipients JSON,
            parents JSON,
            owner_id BIGINT,
            server_id BIGINT,
            FOREIGN KEY (owner_id) REFERENCES users(id)
            FOREIGN KEY (server_id) REFERENCES servers(id)
        )`
    }
}