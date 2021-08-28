import { UUID } from '../utils'

export interface CreateRoleOptions extends Partial<Role> {
    name: string   
}

export class Role {
    _id!: string
    name!: string
    permissions = 0
    color?: number
    hoist = false
    static from(options: CreateRoleOptions): Role {
        const role = new Role()

        role._id = UUID.generate()

        Object.assign(role, options)

        return role
    }
}