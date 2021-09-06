import { Entity, Enum, FindOptions, FilterQuery, Property, wrap } from 'mikro-orm'
import { Base } from './Base'
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

export interface DMChannel extends Channel {
    type: ChannelTypes.DM
    recipients: Snowflake[]
}

export interface Group extends Channel {
    type: ChannelTypes.GROUP
    name: string
    ownerId: Snowflake
    recipients: Snowflake[]
}

export interface TextChannel extends Channel {
    type: ChannelTypes.TEXT
    name: string
    serverId: Snowflake
}

export interface Category extends Channel {
    type: ChannelTypes.CATEGORY
    name: string
    serverId: Snowflake
    channels: Snowflake[]
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
    type = ChannelTypes.UNKNOWN as any

    @Property()
    name!: string

    @Property({ nullable: true })
    topic?: string

    @Property({ nullable: true })
    serverId!: Snowflake

    // Group/DM

    @Property()
    recipients!: Snowflake[]

    @Property()
    ownerId!: Snowflake

    @Property({ nullable: true })
    icon?: string

    @Property()
    permissions = DEFAULT_PERMISSION_DM

    @Property()
    channels!: Snowflake[]

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
        return wrap(new Channel().setID()).assign(options)
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