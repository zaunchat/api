import { Base } from './Base'
import { Entity, Property, wrap } from 'mikro-orm'
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
}
