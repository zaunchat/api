import type { Member, Message, Server, Channel, User } from '../structures'

export interface Payload {
    code: WSCodes
    event?: keyof WSEvents
    data?: unknown
}

export type WithID<T = unknown, Partially extends boolean = true> = Partially extends true
    ? Partial<T> & { _id: ID }
    : T & { _id: ID }


export interface WSEvents {
    READY: {
        user: User
        channels: Channel[]
        servers: Server[]
        users: User[]
    }

    MESSAGE_CREATE: Message
    MESSAGE_DELETE: WithID
    MESSAGE_UPDATE: WithID<Message>

    CHANNEL_CREATE: Channel
    CHANNEL_UPDATE: WithID<Channel>
    CHANNEL_DELETE: WithID<{ server_id?: ID }>

    SERVER_CREATE: Server
    SERVER_DELETE: WithID
    SERVER_UPDATE: WithID<Server>

    MEMBER_JOIN_SERVER: Member
    MEMBER_LEAVE_SERVER: WithID<{ server_id: ID }>
    MEMBER_UPDATE: WithID<Member>

    USER_UPDATE: WithID<User>
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