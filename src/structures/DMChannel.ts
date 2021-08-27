import { Property, wrap } from 'mikro-orm'
import { Channel, ChannelTypes } from './Channel'

export interface CreateDMChannelOptions extends Omit<Partial<DMChannel>, 'type'> {
    recipients: [string, string]
}

export class DMChannel extends Channel {
    @Property()
    readonly type = ChannelTypes.DM

    @Property()
    recipients!: [string, string]

    static from(options: CreateDMChannelOptions): DMChannel {
        return wrap(new DMChannel()).assign(options)
    }
}