import { DBConnection } from '../database'
import { User } from '../structures'


export declare global {
    const db: DBConnection
}

declare module '@tinyhttp/app' {
    interface Request {
        user: User
    }
}