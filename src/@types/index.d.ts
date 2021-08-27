import type { Message, Channel, User } from '../structures'
import ws from 'ws'

declare module '@tinyhttp/app' {
    interface Request {
        user: User
        ws: ws
    }
}


export interface WSEvents {
    READY: {
        channels: Channel[]
        servers: []
        users: User[]
        members: []
    }
    MESSAGE_CREATE: Message
    MESSAGE_DELETE: Pick<Message, '_id' | 'channelId'>
    CHANNEL_CREATE: Channel
    CHANNEL_DELETE: Pick<Channel, '_id'>
}