import { Entity, Property } from 'mikro-orm'
import { Base } from './Base'

export enum ChannelTypes {
    DM,
    TEXT,
    VOICE,
    UNKNOWN
}


@Entity({ tableName: 'channels' })
export class Channel extends Base {
    @Property()
    type = ChannelTypes.UNKNOWN
}