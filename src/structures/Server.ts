import { Base, Role } from '.'
import { Property, Entity, wrap, FindOptions, FilterQuery } from 'mikro-orm'
import { DEFAULT_PERMISSION_EVERYONE } from '../utils'
import db from '../database'

export interface CreateServerOptions extends Partial<Server> {
    name: string
    ownerId: string
}

@Entity({ tableName: 'servers' })
export class Server extends Base {
    @Property()
    name!: string

    @Property({ nullable: true })
    description?: string

    @Property({ nullable: true })
    icon?: string

    @Property({ nullable: true })
    banner?: string

    @Property()
    ownerId!: string

    @Property()
    channels: string[] = []

    @Property()
    roles: Role[] = []

    @Property()
    permissions = DEFAULT_PERMISSION_EVERYONE

    static from(options: CreateServerOptions): Server {
        return wrap(new Server().setID()).assign(options)
    }

    static find(query: FilterQuery<Server>, options?: FindOptions<Server>): Promise<Server[]> {
        return db.get(Server).find(query, options)
    }

    static findOne(query: FilterQuery<Server>): Promise<Server | null> {
        return db.get(Server).findOne(query)
    }

    async save(options?: Partial<Server>): Promise<this> {
        await db.get(Server).persistAndFlush(options ? wrap(this).assign(options) : this)
        return this
    }
}