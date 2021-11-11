import { SyncCheckFunction, AsyncCheckFunction } from 'fastest-validator'
import { User, Server, Channel } from '../structures'
import { Permissions } from '../utils'

declare module '@tinyhttp/app' {
    interface Request {
        user: User
        server: Server
        channel: Channel
        permissions: Permissions
        check(fn: SyncCheckFunction | AsyncCheckFunction): void
    }
}

declare global {
    type ID = `${bigint}`
}