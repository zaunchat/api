import { Entity, Property, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import { Channel, ChannelTypes } from '.'
import { validator } from '../utils'
import db from '../database'
import config from '../../config'

export interface CreateTextChannelOptions extends Omit<Partial<TextChannel>, 'type'> {
    name: string
    serverId: Snowflake
}

export const CreateTextChannelSchema = validator.compile({
    name: {
        type: 'string',
        min: 2,
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

@Entity({ tableName: 'channels' })
export class TextChannel extends Channel {
    @Property()
    readonly type = ChannelTypes.TEXT

    @Property()
    name!: string

    @Property()
    serverId!: Snowflake

    @Property({ nullable: true })
    topic?: string

    static from(options: CreateTextChannelOptions): TextChannel {
        return wrap(new TextChannel().setID()).assign(options)
    }

    static find(query: FilterQuery<TextChannel>, options?: FindOptions<TextChannel>): Promise<TextChannel[]> {
        return db.get(TextChannel).find(query, options)
    }

    static findOne(query: FilterQuery<TextChannel>): Promise<TextChannel | null> {
        return db.get(TextChannel).findOne(query)
    }

    static async save(...channels: TextChannel[]): Promise<void> {
        await db.get(TextChannel).persistAndFlush(channels)
    }

    async save(options?: Partial<TextChannel>): Promise<this> {
        await TextChannel.save(options ? wrap(this).assign(options) : this)
        return this
    }

    async delete(): Promise<void> {
        await db.get(TextChannel).removeAndFlush(this)
    }
}