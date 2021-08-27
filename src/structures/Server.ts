import { Base } from './Base'
import { Property, Entity, wrap } from 'mikro-orm'

export interface CreateServerOptions extends Partial<Server> {
    name: string
    ownerId: string
}

@Entity({ tableName: 'servers' })
export class Server extends Base {
    @Property()
    name!: string

    @Property()
    description?: string

    @Property()
    icon?: string

    @Property()
    banner?: string

    @Property()
    ownerId!: string

    @Property()
    channels: string[] = []
    
    @Property()
    roles: unknown[] = []

    static from(options: CreateServerOptions): Server {
        return wrap(new Server()).assign(options)
    }
}