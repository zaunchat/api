import { Base } from './Base'
import { Property, Entity, wrap } from 'mikro-orm'

export interface CreateMemberOptions extends Partial<Member> {
    _id: string,
    serverId: string
}

@Entity({ tableName: 'members' })
export class Member extends Base {
    @Property({ nullable: true })
    nickname?: string

    @Property({ onCreate: () => Date.now() })
    joinedTimestamp!: number

    @Property()
    roles: string[] = []

    @Property()
    serverId!: string

    @Property()
    userId!: string

    static from(options: CreateMemberOptions): Member {
        return wrap(new Member()).assign(options)
    }
}