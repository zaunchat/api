import { Base, Session, Server, Member } from '.'
import { validator } from '../utils'
import sql from '../database'
import config from '../config'
import { Bot } from './Bot'

export type PublicUser = Omit<User, 'email' | 'password' | 'relations' | 'verified'>

export const PUBLIC_USER_PROPS: (keyof PublicUser)[] = [
  'id',
  'username',
  'presence',
  'badges',
  'avatar'
]


export interface CreateUserOptions extends Partial<User> {
  username: string
  password: string
  email: string
}

export const CreateUserSchema = validator.compile({
  $$async: true,
  username: {
    type: 'string',
    min: 3,
    max: config.limits.user.username,
    pattern: /^[a-z0-9_]+$/i,
    custom: async (value: string, errors: unknown[]) => {
      if (['system', 'admin', 'bot', 'developer', 'staff', '___'].includes(value.toLowerCase())) {
        errors.push({ type: 'unique', actual: value })
      } else {
        const exists = await User.findOne({ username: value })
        if (exists) errors.push({ type: "unique", actual: value })
      }
      return value
    }
  },
  email: {
    type: 'email',
    normalize: true,
    custom: async (value: string, errors: unknown[]) => {
      const exists = await User.findOne({ email: value })
      if (exists) errors.push({ type: "unique", actual: value })
      return value
    }
  },
  password: 'string|min:8|max:72'
})

export const LoginUserSchema = validator.compile({
  email: 'email|normalize',
  password: 'string|min:8|max:32'
})

export const LogoutUserSchema = validator.compile({
  token: 'string',
  user_id: 'string'
})

export interface Presence {
  text?: string
  status: PresenceStatus
}

export type Relationships = Record<string, RelationshipStatus>

export enum PresenceStatus {
  ONLINE,
  OFFLINE,
  IDLE,
  DND
}

export enum RelationshipStatus {
  FRIEND,
  OUTGOING,
  IN_COMING,
  BLOCKED,
  BLOCKED_BY_OTHER
}



export class User extends Base {
  username!: string
  password!: string
  email!: string
  presence: Presence = { status: PresenceStatus.OFFLINE }
  relations: Record<string, RelationshipStatus> = {}
  badges = 0n
  avatar: Nullable<string> = null
  verified = false

  static from(opts: CreateUserOptions): User {
    return Object.assign(new User(), opts)
  }

  static fetchPublicUser(id: string): Promise<PublicUser> {
    return User.findOne(sql => sql.select(PUBLIC_USER_PROPS).where({ id }))
  }

  fetchServers(): Promise<Server[]> {
    return sql<Server[]>`SELECT * FROM ${sql(Server.tableName)} WHERE id IN (
       SELECT server_id FROM members WHERE id = ${this.id}
    )`
  }

  fetchSessions(): Promise<Session[]> {
    return Session.find({ user_id: this.id })
  }

  fetchRelations(): Promise<PublicUser[]> {
    return User.find(sql => sql
      .select(PUBLIC_USER_PROPS)
      .where({ id: Object.keys(this.relations) }))
  }

  fetchBots(): Promise<Bot[]> {
    return Bot.find({ owner_id: this.id })
  }

  async member(server: Server | string): Promise<Member> {
    const server_id = typeof server === 'string' ? server : server.id
    return await Member.findOne({ id: this.id, server_id })
  }

  static async fetchByToken(token: string): Promise<Nullable<User>> {
    const [user]: [User?] = await sql`
         SELECT *
         FROM ${sql(this.tableName)}
         LEFT JOIN sessions
         ON sessions.user_id = users.id
         WHERE verified = TRUE 
         AND sessions.token = ${token}
    `
    return user ?? null
  }
}
