import { Channel, ChannelTypes } from './Channel'
import { Property, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import { validator } from '../utils'
import db from '../database'

export interface CreateCategoryOptions extends Omit<Partial<Category>, 'type'> {
    name: string
    serverId: string
}


export const CreateCategorySchema = validator.compile({
    name: {
        type: 'string',
        min: 2,
        max: 50
    }
})

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

    static async save(...categorys: Category[]): Promise<void> {
        await db.get(Category).persistAndFlush(categorys)
    }

    async save(options?: Partial<Category>): Promise<this> {
        await Category.save(options ? wrap(this).assign(options) : this)
        return this
    }
}