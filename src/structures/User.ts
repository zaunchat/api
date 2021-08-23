import { Base } from './Base'
import { Entity, Property, wrap } from 'mikro-orm'
import { Session } from './Session'

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

@Entity()
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
    friends: User[] = []

    @Property()
    servers: unknown[] = []

    @Property({ nullable: true })
    avatar?: string

    @Property()
    sessions: Session[] = []

    static from(options: CreateUserOptions): User {
        return wrap(new User()).assign(options)
    }
}
