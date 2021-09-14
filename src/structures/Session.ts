import { Property, Entity, wrap } from '@mikro-orm/core'
import { nanoid } from 'nanoid'
import { Base } from './Base'

export interface CreateSessionOptions extends Partial<Session> {
    name?: string
}

@Entity({ tableName: 'sessions' })
export class Session extends Base {
    @Property()
    token: string = nanoid(64)

    @Property({ nullable: true })
    name?: string

    static from(options: CreateSessionOptions): Session {
        return wrap(new Session().setID()).assign(options)
    }
}