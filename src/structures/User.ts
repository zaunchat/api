import { Entity, Property, wrap, FindOptions, FilterQuery, FindOneOptions } from 'mikro-orm'
import db from '../database'
import { Base } from './Base'
import { Session } from './Session'

export enum RelationshipStatus {
    USER,
    FRIEND,
    OUTGOING,
    IM_COMING,
    BLOCKED,
    BLOCKED_OTHER,
}

export interface Relationship {
    id: string
    status: RelationshipStatus
}

export enum UserBadges {
    DEVELOPER,
    TRANSLATOR,
    SUPPORTER
}


export interface CreateUserOptions extends Partial<User> {
    username: string
    password: string
    email: string
}

@Entity({ tableName: 'users' })
export class User extends Base {
    @Property({ unique: true })
    username!: string

    @Property()
    password!: string

    @Property({ unique: true })
    email!: string

    @Property()
    badges = 0

    @Property()
    relations: Relationship[] = []

    @Property()
    servers: string[] = []

    @Property({ nullable: true })
    avatar?: string

    @Property()
    sessions: Session[] = []

    @Property()
    verified = false

    static from(options: CreateUserOptions): User {
        return wrap(new User().setID()).assign(options)
    }

    static find(query: FilterQuery<User>, options?: FindOptions<User>): Promise<User[]> {
        return db.get(User).find(query, options)
    }

    static findOne(query: FilterQuery<User>, options?: FindOneOptions<User>): Promise<User | null> {
        return db.get(User).findOne(query, options)
    }

    static count(query: FilterQuery<User>): Promise<number> {
        return db.get(User).count(query)
    }

    static remove(user: User): Promise<void> {
        return db.get(User).removeAndFlush(user)
    }

    async save(options?: Partial<User>): Promise<void> {
        await db.get(User).persistAndFlush(options ? wrap(this).assign(options) : this)
    }
}
