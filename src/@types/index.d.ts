import { SyncCheckFunction, AsyncCheckFunction } from 'fastest-validator'
import { APIErrors } from '../errors'
import { User } from '../structures'
import { Permissions } from '../utils'

declare module '@tinyhttp/app' {
  interface Request {
    permissions: Permissions
    user: User
    check(fn: SyncCheckFunction | AsyncCheckFunction): void
    throw(tag: keyof typeof APIErrors): void
  }
}

declare global {
  type ID = string
}
