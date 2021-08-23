import { Property } from 'mikro-orm'
import { Channel, ChannelTypes } from './Channel'

export class DMChannel extends Channel {
    @Property()
    readonly type = ChannelTypes.DM
    
    @Property()
    userId!: string

    @Property()
    recipients!: string
}