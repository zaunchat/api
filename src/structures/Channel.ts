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
    owner: User
    permissions: number
}

export interface TextChannel extends Channel {
    type: ChannelTypes.TEXT
    name: string
    server: Server
    overwrites: ChannelOverwrites
}

export interface Category extends Channel {
    type: ChannelTypes.CATEGORY
    name: string
    server: Server
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



export class Channel extends Base implements DMChannel, Group, TextChannel, Category {
    // deno-lint-ignore no-explicit-any]
    type = ChannelTypes.UNKNOWN as any
    name!: string
    topic?: string
    server!: Server
    overwrites!: ChannelOverwrites
    owner!: User
    icon?: string
    permissions = DEFAULT_PERMISSION_DM
    parents!: ID[]

    static toSQL(): string {
        return `CREATE TABLE channels IF NOT EXISTS (
            id BIGINT NOT NULL,
            type INTEGER NOT NULL,
        )`
    }
}