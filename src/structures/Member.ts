import { Base } from './Base'
import { Property, Entity, wrap, FilterQuery, FindOptions } from 'mikro-orm'
import { validator } from '../utils'
import db from '../database'

export interface CreateMemberOptions extends Partial<Member> {
    _id: string,
    serverId: string
}

export const CreateMemberSchema = validator.compile({
    nickname: {
        type: 'string',
        min: 1,
        max: 30,
        optional: true
    }
})


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

    static from(options: CreateMemberOptions): Member {
        return wrap(new Member()).assign(options)
    }

    static find(query: FilterQuery<Member>, options?: FindOptions<Member>): Promise<Member[]> {
        return db.get(Member).find(query, options)
    }

    static findOne(query: FilterQuery<Member>): Promise<Member | null> {
        return db.get(Member).findOne(query)
    }

    static async save(...members: Member[]): Promise<void> {
        await db.get(Member).persistAndFlush(members)
    }

    async save(options?: Partial<Member>): Promise<this> {
        await Member.save(options ? wrap(this).assign(options) : this)
        return this
    }
}