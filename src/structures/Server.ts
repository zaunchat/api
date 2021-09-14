import { Base, Role, User, Channel } from '.'
import { Property, Entity, wrap, FindOptions, FilterQuery, ManyToMany, Collection, OneToOne } from '@mikro-orm/core'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
import db from '../database'
import config from '../../config'

export interface CreateServerOptions extends Partial<Server> {
    name: string
    owner: User
}

export const CreateServerSchema = validator.compile({
    name: {
        type: 'string',
        min: 1,
        max: config.limits.server.name
    }
})

export const ModifyServerSchema = validator.compile({
    name: {
        type: 'string',
        min: 1,
        max: config.limits.server.name,
        optional: true
    },
    description: {
        type: 'string',
        min: 0,
        max: config.limits.server.description,
        optional: true
    }
})


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

    @OneToOne({ entity: () => User })
    owner!: User

    @ManyToMany({ entity: () => Role })
    roles = new Collection<Role>(this)

    @ManyToMany({ entity: () => Channel })
    channels = new Collection<Channel>(this)

    @Property()
    permissions: number = DEFAULT_PERMISSION_EVERYONE

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

    async delete(): Promise<void> {
        await db.get(Server).removeAndFlush(this)
    }
}