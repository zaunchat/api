import { Base, Role, Server } from '.'
import { Property, Entity, wrap, FilterQuery, FindOptions, ManyToMany, Collection, OneToOne } from 'mikro-orm'
import { validator } from '../utils'
import db from '../database'
import config from '../../config'

export interface CreateMemberOptions extends Partial<Member> {
    _id: ID,
    server: Server
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
    joined_timestamp!: number

    @ManyToMany('Role')
    roles = new Collection<Role>(this)

    @OneToOne('Server')
    server!: Server

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

    async delete(): Promise<void> {
        await db.get(Member).removeAndFlush(this)
    }
}