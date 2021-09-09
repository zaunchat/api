import { SyncCheckFunction, AsyncCheckFunction } from 'fastest-validator'
import { User } from '../structures'

declare module '@tinyhttp/app' {
    interface Request {
        user: User
        check(fn: SyncCheckFunction | AsyncCheckFunction): boolean
    }

    interface Response {
        ok(): void
    }
}

declare global {
    type ID = `${bigint}`
}