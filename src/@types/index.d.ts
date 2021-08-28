import type { Message, Channel, User, Server, DMChannel } from '../structures'
import ws from 'ws'

declare module '@tinyhttp/app' {
    interface Request {
        user: User
        ws: ws
    }
}

type If<T extends boolean, A, B = null> = T extends true ? A : T extends false ? B : A | B;

interface WSEvents {
    READY: {
        user: User
        channels: Channel[]
        servers: Server[]
        users: User[]
        members: []
    }
    MESSAGE_CREATE: Message
    MESSAGE_DELETE: Pick<Message, '_id' | 'channelId'>
    CHANNEL_CREATE: Channel
    CHANNEL_DELETE: Pick<TextChannel, '_id', 'serverId'> | Pick<DMChannel, '_id'>
    SERVER_CREATE: Server
    SERVER_DELETE: Pick<Server, '_id'>
}