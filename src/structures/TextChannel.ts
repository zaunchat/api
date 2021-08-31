import { Entity, Property, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import { Channel, ChannelTypes } from '.'
import db from '../database'

export interface CreateTextChannelOptions extends Omit<Partial<TextChannel>, 'type'> {
    name: string
    serverId: string
}

@Entity({ tableName: 'channels' })
export class TextChannel extends Channel {
    @Property()
    readonly type = ChannelTypes.TEXT

    @Property()
    name!: string

    @Property()
    serverId!: string

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

    async save(options?: Partial<TextChannel>): Promise<this> {
        await db.get(TextChannel).persistAndFlush(options ? wrap(this).assign(options) : this)
        return this
    }
}