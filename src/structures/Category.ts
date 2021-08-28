import { Channel, ChannelTypes } from './Channel'
import { Property, wrap } from 'mikro-orm'


export interface CreateCategoryOptions extends Omit<Partial<Category>, 'type'> {
    name: string
    serverId: string
}

export class Category extends Channel {
    @Property()
    readonly type = ChannelTypes.CATEGORY

    @Property()
    name!: string

    @Property()
    serverId!: string

    @Property()
    channels: string[] = []

    static from(options: CreateCategoryOptions): Category {
        return wrap(new Category().setID()).assign(options)
    }
}