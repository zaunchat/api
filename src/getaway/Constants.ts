import type { Member, Message, Server, Channel, User } from '../structures'

export interface Payload {
    code: WSCodes
    event?: keyof WSEvents
    data?: unknown
}


export interface WSEvents {
    READY: {
        user: User
        channels: Channel[]
        servers: Server[]
        users: User[]
    }
    
    MESSAGE_CREATE: Message
    MESSAGE_DELETE: Pick<Message, '_id' | 'channel'>
    MESSAGE_UPDATE: Partial<Message> & { _id: ID }
    
    CHANNEL_CREATE: Channel
    CHANNEL_UPDATE: Partial<Channel> & { _id: ID }
    CHANNEL_DELETE: { _id: ID } & { server_id?: ID }

    SERVER_CREATE: Server
    SERVER_DELETE: { _id: ID }
    SERVER_UPDATE: Partial<Server> & { _id: ID }
    
    MEMBER_JOIN_SERVER: Member
    MEMBER_LEAVE_SERVER: { _id: ID } & { server_id: ID }
    MEMBER_UPDATE: Partial<Member>

    USER_UPDATE: Partial<User> & { _id: ID }
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