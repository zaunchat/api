import { Entity, Enum } from 'mikro-orm'
import { Base } from './Base'

export enum ChannelTypes {
    DM,
    TEXT,
    VOICE,
    CATEGORY,
    GROUP,
    UNKNOWN
}


@Entity({ tableName: 'channels', abstract: true })
export abstract class Channel extends Base {
    @Enum(() => ChannelTypes)
    type = ChannelTypes.UNKNOWN
}