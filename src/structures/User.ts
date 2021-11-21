import { Base, Session, Server } from '.'
import { validator } from '../utils'
import { HTTPError } from '../errors'
import { getaway } from '../getaway'
import sql from '../database'
import config from '../config'

export const PUBLIC_USER_PROPS = [
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
  username: {
    type: 'string',
    min: 3,
    max: config.limits.user.username
  },
  email: {
    type: 'string',
    min: 3,
    max: 320
  },
  password: {
    type: 'string',
    min: 8,
    max: 72
  }
})

export const LoginUserSchema = validator.compile({
  email: { type: 'string', min: 3, max: 320 },
  password: { type: 'string', min: 8, max: 72 }
})

export const LogoutUserSchema = validator.compile({
  token: { type: 'string' },
  user_id: { type: 'string' }
})

export interface Presence {
  text?: string
  status: PresenceStatus
}

export interface Relationship {
  id: ID,
  status: RelationshipStatus
}

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
  BLOCKED_OTHER
}


export class User extends Base {
  username!: string
  password!: string
  email!: string
  presence: Presence = { status: PresenceStatus.OFFLINE }
  relations: Relationship[] = []
  badges = 0
  avatar: string | null = null
  verified = false

  static async onUpdate(self: User): Promise<void> {
    // TODO: Better handling
    await getaway.publish(self.id, 'USER_UPDATE', {
      id: self.id,
      avatar: self.avatar,
      badges: self.badges,
      username: self.username,
      presence: self.presence
    })
  }

  static from(opts: CreateUserOptions): User {
    return Object.assign(new User(), opts)
  }

  static async find(where: string, select: (keyof User | '*')[] = ['*'], limit = 100): Promise<User[]> {
    const result: User[] = await sql.unsafe(`
        SELECT ${select} FROM ${this.tableName}
        WHERE ${where}
        LIMIT ${limit}
    `)
    return result.map((row) => User.from(row))
  }

  static async findOne(where: string, select: (keyof User | '*')[] = ['*']): Promise<User> {
    const [user]: [User?] = await sql.unsafe(`SELECT ${select} FROM ${this.tableName} WHERE ${where}`)

    if (user) return User.from(user)

    throw new HTTPError('UNKNOWN_USER')
  }

  fetchServers(): Promise<Server[]> {
    return sql<Server[]>`SELECT * FROM servers WHERE id IN (
            SELECT server_id FROM members WHERE id = ${this.id}
        )`.then(result => result.map(row => Server.from(row)))
  }

  fetchSessions(): Promise<Session[]> {
    return sql<Session[]>`SELECT * FROM sessions WHERE user_id = ${this.id}`.then(res => res.map(Session.from))
  }

  fetchRelations(): Promise<User[]> {
    return sql<User[]>`SELECT * FROM users WHERE id IN (${[...this.relations.keys()]})`.then(res => res.map(User.from))
  }

  static async fetchByToken(token: string): Promise<User | null> {
    const [user]: [User?] = await sql`
         SELECT *
         FROM users 
         LEFT JOIN sessions
         ON sessions.user_id = users.id
         WHERE verified = TRUE 
         AND sessions.token = ${token}
    `

    if (!user) return null

    return User.from(user)
  }

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            username VARCHAR(${config.limits.user.username}) NOT NULL,
            password VARCHAR(32) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            avatar VARCHAR(64),
            badges INTEGER DEFAULT 0,
            presence JSON NOT NULL,
            relations JSON NOT NULL,
            verified BOOLEAN DEFAULT FALSE
        )`)
  }
}
