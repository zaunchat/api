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

    static toObject(channel: Channel): unknown {
        const response: Record<string, unknown> = {
            id: channel._id,
            type: ChannelTypes.UNKNOWN,
            deleted: channel.deleted
        }

        let c: DMChannel
        
        switch (channel.type) {
            case ChannelTypes.DM:
                c = channel as DMChannel
                response.type = ChannelTypes.DM
                response.userId = c.userId
                response.recipients = c.recipients
                break
        }

        return response
    }
}