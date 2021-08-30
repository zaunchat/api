import { Channel, ChannelTypes } from './Channel'
import { Property, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import db from '../database'


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

    static find(query: FilterQuery<Category>, options?: FindOptions<Category>): Promise<Category[]> {
        return db.get(Category).find(query, options)
    }

    static findOne(query: FilterQuery<Category>): Promise<Category | null> {
        return db.get(Category).findOne(query)
    }

    async save(options?: Partial<Category> ): Promise<void> {
        await db.get(Category).persistAndFlush(options ? wrap(this).assign(options) : this)
    }
}