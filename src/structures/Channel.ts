import { Base, User, Server } from '.'
import { DEFAULT_PERMISSION_DM, validator } from '../utils'
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
    static toSQL(): string {
        return `CREATE TABLE IF NOT EXISTS channels (
            id BIGINT PRIMARY KEY,
            type INTEGER NOT NULL,
            name VARCHAR(${config.limits.channel.name}),
            topic VARCHAR(${config.limits.channel.topic}),
            permissions BIGINT DEFAULT 0,
            overwrites JSON,
            parents JSON,
            owner_id BIGINT,
            server_id BIGINT,
            FOREIGN KEY (owner_id) REFERENCES users(id)
            FOREIGN KEY (server_id) REFERENCES servers(id)
        )`
    }
}