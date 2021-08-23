import { User } from '../structures'
import { EntityManager } from 'mikro-orm'

export declare global {
    const db: EntityManager
}

declare module '@tinyhttp/app' {
    interface Request {
        user: User
    }
}