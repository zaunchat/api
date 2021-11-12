
import { nanoid } from 'nanoid'
import { Base } from './Base'

export interface CreateSessionOptions extends Partial<Session> {
    name?: string
}


export class Session extends Base {
    token = nanoid(64)
    name?: string
}