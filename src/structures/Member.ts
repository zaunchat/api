import { Base } from './Base'
import { Property, Entity, wrap } from 'mikro-orm'

export interface CreateMemberOptions extends Partial<Member> {
    serverId: string
}

@Entity({ tableName: 'members' })
export class Member extends Base {
    @Property()
    nickname?: string

    @Property()
    roles: unknown[] = []

    @Property()
    serverId!: string
    
    static from(options: CreateMemberOptions): Member {
        return wrap(new Member()).assign(options)
    }
}