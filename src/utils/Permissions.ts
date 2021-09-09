import { ChannelTypes, DMChannel, Group, Member, Server, TextChannel, User, Category } from '../structures'


export type PermissionString = keyof typeof Permissions.FLAGS
export type PermissionsResolvable = number | PermissionString | Permissions | PermissionsResolvable[]


export class Permissions {
    static readonly DEFAULT_BIT = 0
    static readonly FLAGS = {
        // Admin
        ADMINISTRATOR: 1 << 0,


        // Channel
        VIEW_CHANNEL: 1 << 1,
        SEND_MESSAGES: 1 << 2,
        READ_MESSAGE_HISTORY: 1 << 3,
        EMBED_LINKS: 1 << 4,
        UPLOAD_FILES: 1 << 5,


        // Manage
        MANAGE_SERVER: 1 << 6,
        MANAGE_CHANNELS: 1 << 7,
        MANAGE_MESSAGES: 1 << 8,
        MANAGE_ROLES: 1 << 9,
        MANAGE_NICKNAMES: 1 << 10,
        BAN_MEMBERS: 1 << 11,
        KICK_MEMBERS: 1 << 12,


        // Member
        CHANGE_NICKNAME: 1 << 13
    }

    bitfield = Permissions.DEFAULT_BIT

    constructor(...bits: PermissionsResolvable[]) {
        this.bitfield = Permissions.resolve(bits)
    }

    static async fetch(user: User | string, server?: Server | string | null, channel?: DMChannel | Group | TextChannel | Category): Promise<Permissions> {
        user = typeof user === 'string' ? await User.findOne({ _id: user as ID }) as User : user

        const permissions = new Permissions()


        if (server) {
            let member: Member

            [member, server] = await Promise.all([
                Member.findOne({ _id: typeof user === 'string' ? user : user._id }),
                typeof server === 'string' ? Server.findOne({ _id: server }) : server,
            ]) as [Member, Server]

            if (member._id === server.owner._id) {
                return permissions.add(Permissions.FLAGS.ADMINISTRATOR)
            } else {
                permissions.add(server.permissions)
                for (const role of server.roles.getItems()) {
                    if (member.roles.contains(role)) permissions.add(role.permissions)
                }
            }
        }

        if (permissions.has(Permissions.FLAGS.ADMINISTRATOR, false)) {
            return permissions
        }

        if (channel) {
            switch (channel.type) {
                case ChannelTypes.GROUP:
                    if (channel.owner._id === user._id) {
                        permissions.add(Permissions.FLAGS.ADMINISTRATOR)
                    } else if (channel.recipients.contains(user)) {
                        permissions.add(channel.permissions)
                    }
                    break
                case ChannelTypes.DM:
                    if (channel.recipients.contains(user)) permissions.add(DEFAULT_PERMISSION_DM)
                    break
                case ChannelTypes.TEXT:
                case ChannelTypes.CATEGORY:
                    break // Todo: Handle channel overwrites
                default:
                    throw new Error(`Unknown channel type - ${channel}`)
            }
        }

        return permissions
    }

    missing(bits: PermissionsResolvable, checkAdmin = true): PermissionString[] {
        if (checkAdmin && this.has(Permissions.FLAGS.ADMINISTRATOR, false)) return []
        return new Permissions(bits).remove(this).toArray()
    }

    any(bit: PermissionsResolvable, checkAdmin = true): boolean {
        if (checkAdmin && this.has(Permissions.FLAGS.ADMINISTRATOR, false)) return true
        bit = Permissions.resolve(bit)
        return (this.bitfield & bit) !== Permissions.DEFAULT_BIT
    }


    has(bit: PermissionsResolvable, checkAdmin = true): boolean {
        if (checkAdmin && this.has(Permissions.FLAGS.ADMINISTRATOR, false)) return true
        bit = Permissions.resolve(bit)
        return (this.bitfield & bit) === bit
    }


    add(...bits: PermissionsResolvable[]): Permissions {
        let total = 0

        for (const bit of bits) {
            total |= Permissions.resolve(bit)
        }

        if (Object.isFrozen(this)) return new Permissions(this.bitfield | total)

        this.bitfield |= total

        return this
    }

    remove(...bits: PermissionsResolvable[]): Permissions {
        let total = 0

        for (const bit of bits) {
            total |= Permissions.resolve(bit)
        }

        if (Object.isFrozen(this)) return new Permissions(this.bitfield & ~total)

        this.bitfield &= ~total

        return this
    }

    freeze(): Readonly<this> {
        return Object.freeze(this)
    }

    valueOf(): number {
        return this.bitfield
    }

    serialize(): Record<string, boolean> {
        const serialized: Record<string, boolean> = {}
        for (const [flag, bit] of Object.entries(Permissions.FLAGS)) serialized[flag] = this.has(bit)
        return serialized
    }

    toArray(): PermissionString[] {
        const flags = Object.keys(Permissions.FLAGS) as PermissionString[]
        return flags.filter(bit => this.has(bit))
    }

    equals(bit: PermissionsResolvable): boolean {
        return this.bitfield === Permissions.resolve(bit)
    }

    *[Symbol.iterator](): Iterable<PermissionString> {
        yield* this.toArray()
    }

    static resolve(bit: PermissionsResolvable): number {
        if (typeof bit === 'number') return bit
        if (Array.isArray(bit)) return bit.map(p => this.resolve(p)).reduce((prev, p) => prev | p, Permissions.DEFAULT_BIT)
        if (bit instanceof Permissions) return bit.bitfield
        if (typeof Permissions.FLAGS[bit] !== 'undefined') return Permissions.FLAGS[bit]
        throw new Error('Invalid Bit')
    }
}


export const DEFAULT_PERMISSION_DM = new Permissions([
    'VIEW_CHANNEL',
    'SEND_MESSAGES',
    'EMBED_LINKS',
    'UPLOAD_FILES',
    'READ_MESSAGE_HISTORY'
]).bitfield

export const DEFAULT_PERMISSION_EVERYONE = new Permissions([
    'VIEW_CHANNEL',
    'SEND_MESSAGES',
    'EMBED_LINKS',
    'UPLOAD_FILES',
    'READ_MESSAGE_HISTORY'
]).bitfield