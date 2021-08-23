import { nanoid } from 'nanoid'

export interface CreateSessionOptions {
    name?: string
}

export class Session {
    token!: string
    name?: string
    static from({ name }: CreateSessionOptions): Session {
        const session = new Session()
        
        session.name = name
        session.token = nanoid(64)

        return session
    } 
}