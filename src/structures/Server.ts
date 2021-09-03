import { Base, Role, User, Member } from '.'
import { Property, Entity, wrap, FindOptions, FilterQuery } from 'mikro-orm'
import { DEFAULT_PERMISSION_EVERYONE, validator } from '../utils'
import db from '../database'

export interface CreateServerOptions extends Partial<Server> {
    name: string
    ownerId: Snowflake
}

export const CreateServerSchema = validator.compile({
    name: {
        type: 'string',
        min: 2,
        max: 50
    }
})

export const ModifyServerSchema = validator.compile({
    name: {
        type: 'string',
        min: 2,
        max: 50,
        optional: true
    },
    description: {
        type: 'string',
        min: 0,
        max: 1000,
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

    @Property()
    ownerId!: Snowflake

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

    async addMember(user: User): Promise<void> {
        user.servers.push(this._id)
        await Member.from({
            _id: user._id,
            serverId: this._id
        }).save()
    }
}