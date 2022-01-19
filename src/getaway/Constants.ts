import type { Member, Message, Server, Channel, User, PublicUser } from '../structures'
import ms from 'ms'

export const DEFAULT_HEARTBEAT_TIME = ms('45ms')

export interface Payload {
  code: WSCodes
  event?: keyof WSEvents
  data?: unknown
}

type Partial<T extends Empty> = { [P in keyof T]?: T[P] } & Empty
type Empty = { id: string }

export interface WSEvents {
  READY: {
    user: User
    channels: Channel[]
    servers: Server[]
    users: PublicUser[]
  }

  MESSAGE_CREATE: Message
  MESSAGE_UPDATE: Partial<Message>
  MESSAGE_DELETE: Empty

  CHANNEL_CREATE: Channel
  CHANNEL_UPDATE: Partial<Channel>
  CHANNEL_DELETE: Empty & { server_id?: string }

  SERVER_CREATE: Server
  SERVER_UPDATE: Partial<Server>
  SERVER_DELETE: Empty

  SERVER_MEMBER_JOIN: Member
  SERVER_MEMBER_UPDATE: Partial<Member>
  SERVER_MEMBER_LEAVE: Empty & { server_id: string }

  GROUP_MEMBER_JOIN: User
  GROUP_MEMBER_UPDATE: Partial<User>
  GROUP_MEMBER_LEAVE: Empty

  USER_UPDATE: Partial<User>
}

export enum WSCodes {
  HELLO,
  PING,
  PONG,
  AUTHENTICATE,
  AUTHENTICATED,
  READY
}


export enum WSCloseCodes {
  UNKNOWN_ERROR = 4000,
  UNKNOWN_OPCODE,
  DECODE_ERROR,
  NOT_AUTHENTICATED,
  AUTHENTICATED_FAILED,
  ALREADY_AUTHENTICATED,
  INVALID_SESSION,
  RATE_LIMITED,
  SESSION_TIMEOUT
}
