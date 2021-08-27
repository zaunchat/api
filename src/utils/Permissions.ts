import { DMChannel, User } from '../structures'

export const FLAGS = {
    VIEW_CHANNEL: 1 << 0,
    SEND_MESSAGES: 1 << 1,
    READ_MESSAGES: 1 << 2,
    UPLOAD_FILES: 1 << 3,
    EMBED_LINKS: 1 << 4,
    MANAGE_CHANNEL: 1 << 5,
    MANAGE_MESSAGES: 1 << 6
} as const

export const DEFAULT_PERMISSION_DM =
    FLAGS.VIEW_CHANNEL +
    FLAGS.READ_MESSAGES +
    FLAGS.SEND_MESSAGES +
    FLAGS.UPLOAD_FILES +
    FLAGS.MANAGE_CHANNEL +
    FLAGS.EMBED_LINKS


export type PermissionsResolvable = number | keyof typeof FLAGS | PermissionsResolvable[]
export type PermissionsType = 'CHANNEL' | 'USER' | 'SERVER'

export class Permissions {
    bitfield = 0
    perspective!: User

    constructor(public type: PermissionsType) { }

    for(obj: unknown): this {
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        let channel: DMChannel, user: User, server: unknown

        switch (this.type) {
            case 'CHANNEL': {
                channel = obj as DMChannel
                if (channel.recipients.includes(this.perspective._id)) {
                    this.bitfield = DEFAULT_PERMISSION_DM
                }
                break
            }

            case 'USER': {
                break
            }

            case 'SERVER': {
                break
            }
        }

        return this
    }

    with(obj: User): this {
        this.perspective = obj
        return this
    }

    any(bit: PermissionsResolvable): boolean {
        bit = Permissions.resolve(bit)
        return (this.bitfield & bit) !== 0
    }


    has(bit: PermissionsResolvable): boolean {
        bit = Permissions.resolve(bit)
        return (this.bitfield & bit) === bit
    }


    add(...bits: PermissionsResolvable[]): this {
        let total = 0

        for (const bit of bits) {
            total |= Permissions.resolve(bit)
        }

        this.bitfield |= total

        return this
    }

    remove(...bits: PermissionsResolvable[]): this {
        let total = 0

        for (const bit of bits) {
            total |= Permissions.resolve(bit)
        }

        this.bitfield &= ~total

        return this
    }

    static resolve(bit: PermissionsResolvable): number {
        if (typeof bit === 'number') return bit
        if (Array.isArray(bit)) return this.resolve(bit)
        if (typeof FLAGS[bit] !== 'undefined') return FLAGS[bit]
        throw new Error('Invalid Bit')
    }
}