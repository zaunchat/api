import { Entity, Enum, FindOptions, FilterQuery } from 'mikro-orm'
import { Base } from './Base'
import db from '../database'

export enum ChannelTypes {
    DM,
    TEXT,
    VOICE,
    CATEGORY,
    GROUP,
    UNKNOWN = -1
}


@Entity({ tableName: 'channels' })
export abstract class Channel extends Base {
    @Enum(() => ChannelTypes)
    abstract type = ChannelTypes.UNKNOWN

    static findOne(query: FilterQuery<Channel>): Promise<Channel | null> {
        return db.get(Channel).findOne(query)
    }

    static find(query: FilterQuery<Channel>, options?: FindOptions<Channel>): Promise<Channel[]> {
        return db.get(Channel).find(query, options)
    }
}