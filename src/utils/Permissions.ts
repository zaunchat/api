import { ChannelTypes, DMChannel, Group, Member, Server, TextChannel, User } from '../structures'

export const FLAGS = {
    VIEW_CHANNEL: 1 << 0,
    SEND_MESSAGES: 1 << 1,
    READ_MESSAGES: 1 << 2,
    UPLOAD_FILES: 1 << 3,
    EMBED_LINKS: 1 << 4,
    MANAGE_CHANNELS: 1 << 5,
    MANAGE_MESSAGES: 1 << 6,
    ADMINISTRATOR: 1 << 7,
    READ_MESSAGE_HISTORY: 1 << 8
} as const


export const DEFAULT_PERMISSION_DM =
    FLAGS.READ_MESSAGES |
    FLAGS.SEND_MESSAGES |
    FLAGS.MANAGE_CHANNELS |
    FLAGS.EMBED_LINKS |
    FLAGS.VIEW_CHANNEL |
    FLAGS.UPLOAD_FILES |
    FLAGS.READ_MESSAGE_HISTORY

export const DEFAULT_PERMISSION_EVERYONE =
    FLAGS.READ_MESSAGES |
    FLAGS.SEND_MESSAGES |
    FLAGS.EMBED_LINKS |
    FLAGS.VIEW_CHANNEL |
    FLAGS.UPLOAD_FILES |
    FLAGS.READ_MESSAGE_HISTORY


export type PermissionsResolvable = number | keyof typeof FLAGS | PermissionsResolvable[]

export class Permissions {
    bitfield = 0
    perspective!: User

    constructor(...bits: PermissionsResolvable[]) {
        this.bitfield = Permissions.resolve(bits)
    }

    static async fetch(user: User | string, server?: Server | string | null, channel?: DMChannel | Group | TextChannel): Promise<Permissions> {
        if (server) {
            let member: Member

            [member, server] = await Promise.all([
                Member.findOne({ _id: typeof user === 'string' ? user : user._id }),
                typeof server === 'string' ? Server.findOne({ _id: server }) : server,
            ]) as [Member, Server]

            if (member._id === server.ownerId) {
                return new Permissions(FLAGS.ADMINISTRATOR)
            }

            const roles = server.roles.filter((r) => member.roles.includes(r._id)).map((r) => r.permissions)

            roles.push(server.permissions)

            return new Permissions(roles)
        }


        if (channel) {
            const userId = user = typeof user === 'string' ? user : user._id

            let bitfield = 0

            switch (channel.type) {
                case ChannelTypes.GROUP:
                    if (channel.ownerId === userId) {
                        bitfield = FLAGS.ADMINISTRATOR
                    } else if (channel.recipients.some(id => id === userId)) {
                        bitfield = channel.permissions
                    }
                    break
                case ChannelTypes.DM:
                    if (channel.recipients.some(id => id === userId)) bitfield = DEFAULT_PERMISSION_DM
                    break
                case ChannelTypes.TEXT:
                    break
                default:
                    break
            }

            return new Permissions(bitfield)
        }

        return new Permissions()
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