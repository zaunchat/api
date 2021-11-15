import { SyncCheckFunction, AsyncCheckFunction } from 'fastest-validator'
import { User, Server, Channel } from '../structures'
import { Permissions } from '../utils'

declare module '@tinyhttp/app' {
    interface Request {
        permissions: Permissions
        user: User
        check(fn: SyncCheckFunction | AsyncCheckFunction): void
    }
}

declare global {
    type ID = `${bigint}`
}