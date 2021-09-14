import { Entity, Enum, FindOptions, FilterQuery, Property, wrap, ManyToMany, Collection, OneToOne } from '@mikro-orm/core'
import { Base, User, Server } from '.'
import { DEFAULT_PERMISSION_DM, validator } from '../utils'
import db from '../database'
import config from '../../config'

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
    recipients: Collection<User>
}

export interface Group extends Channel {
    type: ChannelTypes.GROUP
    name: string
    owner: User
    recipients: Collection<User>
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
    channels: ID[]
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


@Entity({ tableName: 'channels' })
export class Channel extends Base implements DMChannel, Group, TextChannel, Category {
    @Enum(() => ChannelTypes)
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    type = ChannelTypes.UNKNOWN as any

    @Property()
    name!: string

    @Property({ nullable: true })
    topic?: string

    @OneToOne({ entity: () => Server })
    server!: Server

    @Property()
    overwrites!: ChannelOverwrites

    // Group/DM

    @ManyToMany({ entity: () => User })
    recipients!: Collection<User>

    @OneToOne({ entity: () => User })
    owner!: User

    @Property({ nullable: true })
    icon?: string

    @Property()
    permissions: number = DEFAULT_PERMISSION_DM

    // Category
    @Property()
    channels!: ID[]

    static count(query: FilterQuery<Channel>): Promise<number> {
        return db.get(Channel).count(query)
    }

    static findOne(query: FilterQuery<Channel>): Promise<Channel | null> {
        return db.get(Channel).findOne(query)
    }

    static find(query: FilterQuery<Channel>, options?: FindOptions<Channel>): Promise<Channel[]> {
        return db.get(Channel).find(query, options)
    }

    static from(options: { type: ChannelTypes.TEXT } & Partial<TextChannel>): TextChannel
    static from(options: { type: ChannelTypes.DM } & Partial<DMChannel>): DMChannel
    static from(options: { type: ChannelTypes.GROUP } & Partial<Group>): Group
    static from(options: { type: ChannelTypes.CATEGORY } & Partial<Category>): Category
    static from(options: Partial<DMChannel | Group | TextChannel | Category>): Channel {
        const channel = wrap(new Channel().setID()).assign(options)
        
        if ([ChannelTypes.GROUP, ChannelTypes.DM].includes(channel.type)) {
            channel.recipients = new Collection<User>(channel)
        }

        return channel
    }

    static async save(...channels: Channel[]): Promise<void> {
        await db.get(Channel).persistAndFlush(channels)
    }

    async save(options?: Partial<Channel>): Promise<this> {
        await Channel.save(options ? wrap(this).assign(options) : this)
        return this
    }

    async delete(): Promise<void> {
        await db.get(Channel).removeAndFlush(this)
    }
}