import { Base, Role } from '.'
import { validator } from '../utils'
import { HTTPError } from '../errors'
import { getaway } from '../getaway'
import sql from '../database'
import config from '../config'

export interface CreateMemberOptions extends Partial<Member> {
  id: ID
  server_id: ID
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



export class Member extends Base {
  nickname: string | null = null
  joined_at = Date.now()
  server_id!: ID
  roles: ID[] = []

  static async onCreate(self: Member): Promise<void> {
    await getaway.subscribe(self.id, [self.server_id])
    await getaway.publish(self.server_id, 'MEMBER_JOIN_SERVER', self)
  }

  static async onUpdate(self: Member): Promise<void> {
    await getaway.publish(self.server_id, 'MEMBER_UPDATE', self)
  }

  static async onDelete(self: Member): Promise<void> {
    await getaway.publish(self.server_id, 'MEMBER_LEAVE_SERVER', { id: self.id })
  }

  fetchRoles(): Promise<Role[]> {
    return Role.find(`id IN (${this.roles})`)
  }

  static from(opts: CreateMemberOptions): Member {
    return Object.assign(new Member(), opts)
  }

  static async find(where: string, select: (keyof Member | '*')[] = ['*'], limit = 100): Promise<Member[]> {
    const result: Member[] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where} LIMIT ${limit}`)
    return result.map((row) => Member.from(row))
  }

  static async findOne(where: string, select: (keyof Member | '*')[] = ['*']): Promise<Member> {
    const [member]: [Member?] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where}`)

    if (member) return Member.from(member)

    throw new HTTPError('UNKNOWN_USER')
  }


  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            user_id BIGINT NOT NULL,
            joined_at TIMESTAMP DEFAULT current_timestamp,
            nickname VARCHAR(${config.limits.member.nickname}),
            server_id BIGINT NOT NULL,
            roles JSON NOT NULL,
            FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )`)
  }
}
