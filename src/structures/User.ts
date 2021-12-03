import { Base, Session, Server } from '.'
import { validator } from '../utils'
import { getaway } from '../getaway'
import sql from '../database'
import config from '../config'


export const PUBLIC_USER_PROPS: (keyof User)[] = [
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
  BLOCKED_OTHER
}


export class User extends Base {
  username!: string
  password!: string
  email!: string
  presence: Presence = { status: PresenceStatus.OFFLINE }
  relations: Record<string, RelationshipStatus> = {}
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

  static fetchPublicUser(id: ID): Promise<User> {
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

  fetchRelations(): Promise<User[]> {
    return User.find(sql => sql
      .select(PUBLIC_USER_PROPS)
      .where({ id: Object.keys(this.relations) }))
  }

  with(target: User): RelationshipStatus | null {
    return this.relations[target.id] ?? null
  }

  static async fetchByToken(token: string): Promise<User | null> {
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

  static async init(): Promise<void> {
    await sql.unsafe(`CREATE TABLE IF NOT EXISTS ${this.tableName} (
            id BIGINT PRIMARY KEY,
            username VARCHAR(${config.limits.user.username}) NOT NULL,
            password VARCHAR(32) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            avatar VARCHAR(64),
            badges INTEGER DEFAULT 0,
            presence JSONB NOT NULL,
            relations JSONB NOT NULL,
            verified BOOLEAN DEFAULT FALSE
        )`)
  }
}
