import { Entity, Property, wrap } from 'mikro-orm'
import { Channel, ChannelTypes } from '.'

export interface CreateTextChannelOptions extends Omit<Partial<TextChannel>, 'type'> {
    serverId: string
}

@Entity({ tableName: 'text-channels' })
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
}