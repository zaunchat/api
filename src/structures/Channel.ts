import { Entity, Property } from 'mikro-orm'
import { Base } from './Base'
import { DMChannel } from './DMChannel'

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

    @Property()
    messages: string[] = []

    static hasAccess(userId: string, channel: Channel): boolean {
        let c: DMChannel
        switch (channel.type) {
            case ChannelTypes.DM:
                c = channel as DMChannel
                return c.userId === userId || c.recipients === userId
            default:
                return false
        }
    }
}