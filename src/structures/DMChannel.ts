import { Property, wrap } from 'mikro-orm'
import { Channel, ChannelTypes } from './Channel'

export interface CreateDMChannelOptions extends Omit<Partial<DMChannel>, 'type'> {
    userId: string
    recipients: string
}

export class DMChannel extends Channel {
    @Property()
    readonly type = ChannelTypes.DM
    
    @Property()
    userId!: string

    @Property()
    recipients!: string

    static from(options: CreateDMChannelOptions): DMChannel {
        return wrap(new DMChannel()).assign(options)
    }
}