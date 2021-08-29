import { Entity, Property, wrap } from 'mikro-orm'
import { Channel } from '.'
import { ChannelTypes } from './Channel'

export interface CreateGroupOptions extends Omit<Partial<Group>, 'type'> {
    ownerId: string
    recipients: string[]
}

@Entity({ tableName: 'groups' })
export class Group extends Channel {
    @Property()
    readonly type = ChannelTypes.GROUP

    @Property()
    ownerId!: string

    @Property()
    recipients: string[] = []

    static from(options: CreateGroupOptions): Group {
        return wrap(new Group().setID()).assign(options)
    }
}