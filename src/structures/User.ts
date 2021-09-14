import { Entity, Property, wrap, FindOptions, FilterQuery, FindOneOptions, ManyToMany, Collection } from '@mikro-orm/core'
import { Base, Presence, Session, Server } from '.'
import { validator } from '../utils'
import db from '../database'
import config from '../../config'

export enum RelationshipStatus {
    FRIEND,
    OUTGOING,
    IN_COMING,
    BLOCKED,
    BLOCKED_OTHER
}


export interface CreateUserOptions extends Partial<User> {
    username: string
    password: string
    email: string
}

export const CreateUserSchema = validator.compile({
    username: {
        type: 'string',
        min: 3,
        max: config.limits.user.username
    },
    email: {
        type: 'string',
        min: 3,
        max: 320
    },
    password: {
        type: 'string',
        min: 8,
        max: 72
    }
})

export const LoginUserSchema = validator.compile({
    email: { type: 'string', min: 3, max: 320 },
    password: { type: 'string', min: 8, max: 72 }
})

export const LogoutUserSchema = validator.compile({
    token: { type: 'string' },
    user_id: { type: 'string' }
})

export const PUBLIC_USER_ITEMS: (keyof User)[] = [
    '_id',
    'username',
    'avatar',
    'badges'
]

@Entity({ tableName: 'users' })
export class User extends Base {
    @Property({ unique: true })
    username!: string

    @Property()
    password!: string

    @Property({ unique: true })
    email!: string

    @Property()
    presence: Presence = Presence.from({})

    @Property()
    badges: number = 0

    @Property()
    relations = new Map<ID, RelationshipStatus>()

    @ManyToMany({ entity: () => Server, lazy: true })
    servers = new Collection<Server>(this)

    @Property({ nullable: true })
    avatar?: string

    @ManyToMany({ entity: () => Session })
    sessions = new Collection<Session>(this)

    @Property({ hidden: true })
    verified: boolean = false

    static from(options: CreateUserOptions): User {
        return wrap(new User().setID()).assign(options)
    }

    static find(query: FilterQuery<User>, options?: FindOptions<User> & { public?: boolean }): Promise<User[]> {
        if (options?.public) options.fields = PUBLIC_USER_ITEMS
        return db.get(User).find(query, options)
    }

    static findOne(query: FilterQuery<User>, options?: FindOneOptions<User> & { public?: boolean }): Promise<User | null> {
        if (options?.public) options.fields = PUBLIC_USER_ITEMS
        return db.get(User).findOne(query, options)
    }

    static count(query: FilterQuery<User>): Promise<number> {
        return db.get(User).count(query)
    }

    static remove(user: User): Promise<void> {
        return db.get(User).removeAndFlush(user)
    }

    static async save(...users: User[]): Promise<void> {
        await db.get(User).persistAndFlush(users)
    }

    async save(options?: Partial<User>): Promise<this> {
        await User.save(options ? wrap(this).assign(options) : this)
        return this
    }

    async delete(): Promise<void> {
        await db.get(User).removeAndFlush(this)
    }
}
