import { User } from '../structures'

declare module '@tinyhttp/app' {
    interface Request {
        user: User
    }
}