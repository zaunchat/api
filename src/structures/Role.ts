import { Property, wrap, Entity, FilterQuery, FindOptions, OneToOne } from '@mikro-orm/core'
import { Base, Server } from '.'
import { validator } from '../utils'
import db from '../database'


export interface CreateRoleOptions extends Partial<Role> {
    name: string
    server: Server
}

export const CreateRoleSchema = validator.compile({
    name: {
        type: 'string',
        min: 1,
        max: 30
    },
    color: {
        type: 'number',
        optional: true
    },
    permissions: {
        type: 'number',
        optional: true
    },
    hoist: {
        type: 'boolean',
        optional: true
    }
})

@Entity({ tableName: 'roles' })
export class Role extends Base {
    @Property()
    name!: string

    @Property()
    permissions: number = 0

    @Property({ nullable: true })
    color?: number

    @Property()
    hoist: boolean = false

    @OneToOne({ entity: () => Server })
    server!: Server

    static from(options: CreateRoleOptions): Role {
        return wrap(new Role().setID()).assign(options)
    }

    static find(query: FilterQuery<Role>, options?: FindOptions<Role>): Promise<Role[]> {
        return db.get(Role).find(query, options)
    }

    static findOne(query: FilterQuery<Role>): Promise<Role | null> {
        return db.get(Role).findOne(query)
    }

    async save(options?: Partial<Role>): Promise<this> {
        await db.get(Role).persistAndFlush(options ? wrap(this).assign(options) : this)
        return this
    }

    async delete(): Promise<void> {
        await db.get(Role).removeAndFlush(this)
    }
}