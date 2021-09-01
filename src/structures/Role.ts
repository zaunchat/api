import { UUID, validator } from '../utils'

export interface CreateRoleOptions extends Partial<Role> {
    name: string
}

export const CreateRoleSchema = validator.compile({
    name: {
        type: 'string',
        min: 1,
        max: 30
    },
    color: {
        type: 'number',
        optional: true
    },
    permissions: {
        type: 'number',
        optional: true
    },
    hoist: {
        type: 'boolean',
        optional: true
    }
})

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