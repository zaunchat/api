import { Base, User } from '.'
import { Property, Entity, wrap, FilterQuery, FindOptions, AfterCreate, AfterDelete } from 'mikro-orm'
import { validator } from '../utils'
import db from '../database'
import config from '../../config'

export interface CreateMemberOptions extends Partial<Member> {
    _id: Snowflake,
    serverId: Snowflake
}

export const CreateMemberSchema = validator.compile({
    nickname: {
        type: 'string',
        min: 0,
        max: config.limits.member.nickname,
        optional: true
    },
    roles: {
        type: 'array',
        items: 'string',
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
    serverId!: Snowflake

    @AfterCreate()
    async onCreate(): Promise<void> {
        const user = await User.findOne({ _id: this._id })

        user?.servers.push(this.serverId)

        await user?.save()
    }

    @AfterDelete()
    async onDelete(): Promise<void> {
        const user = await User.findOne({ _id: this._id })
        await user?.save({ servers: user.servers.filter(id => id !== this.serverId) })
    }

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

    async leave(): Promise<void> {
        await db.get(Member).removeAndFlush(this)
    }
}