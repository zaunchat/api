import type { Member, Message, Server, Channel, TextChannel, DMChannel, User, Group } from '../structures'
import { Snowflake } from '../utils'

export interface Payload {
    code: WSCodes
    event?: keyof WSEvents
    data?: unknown
}

export type ID = { _id: Snowflake }

export interface WSEvents {
    READY: {
        user: User
        channels: Channel[]
        servers: Server[]
        users: User[]
        members: Member[]
    }
    
    MESSAGE_CREATE: Message
    MESSAGE_DELETE: Pick<Message, '_id' | 'channelId'>
    MESSAGE_UPDATE: Partial<Pick<Message,| 'channelId' | 'content' | 'attachments' | 'embeds'>> & ID
    
    CHANNEL_CREATE: TextChannel | DMChannel | Group
    CHANNEL_DELETE: ID | Pick<TextChannel, '_id' | 'serverId'>    

    SERVER_CREATE: Server
    SERVER_DELETE: ID
    SERVER_UPDATE: Partial<Pick<Server, 'roles' | 'name' | 'icon' | 'banner' | 'description' | 'ownerId' | 'permissions'>> & ID
    
    MEMBER_JOIN_SERVER: Member
    MEMBER_LEAVE_SERVER: ID & { serverId: Snowflake }
    MEMBER_UPDATE: Partial<Pick<Member, 'nickname' | 'roles'>> & ID & { serverId: Snowflake }

    USER_UPDATE: Partial<Pick<User, 'presence' | 'username' | 'avatar' | 'badges'>> & ID
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