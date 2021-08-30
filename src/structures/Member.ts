import { Base } from './Base'
import { Property, Entity, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import db from '../database'

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

    static find(query: FilterQuery<Member>, options: FindOptions<Member>): Promise<Member[]> {
        return db.get(Member).find(query, options)
    }

    static findOne(query: FilterQuery<Member>): Promise<Member | null> {
        return db.get(Member).findOne(query)
    }

    async save(options?: Partial<Member> ): Promise<void> {
        await db.get(Member).persistAndFlush(options ? wrap(this).assign(options) : this)
    }
}