import { Entity, FindOptions, Property, wrap, FilterQuery } from 'mikro-orm'
import { Channel, ChannelTypes } from './Channel'
import { Snowflake } from '../utils'
import db from '../database'

export interface CreateDMChannelOptions extends Omit<Partial<DMChannel>, 'type'> {
    recipients: [Snowflake, Snowflake]
}


@Entity({ tableName: 'channels' })
export class DMChannel extends Channel {
    @Property()
    readonly type = ChannelTypes.DM

    @Property()
    recipients!: [Snowflake, Snowflake]

    static from(options: CreateDMChannelOptions): DMChannel {
        return wrap(new DMChannel().setID()).assign(options)
    }

    static find(query: FilterQuery<DMChannel>, options?: FindOptions<DMChannel>): Promise<DMChannel[]> {
        return db.get(DMChannel).find(query, options)
    }

    static findOne(query: FilterQuery<DMChannel>): Promise<DMChannel | null> {
        return db.get(DMChannel).findOne(query)
    }

    async save(options?: Partial<DMChannel>): Promise<this> {
        await db.get(DMChannel).persistAndFlush(options ? wrap(this).assign(options) : this)
        return this
    }
}