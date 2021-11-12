import { ChannelTypes, DMChannel, Group, Member, Server, TextChannel, User, Category } from '../structures'
import { BitField } from './BitField'

export type PermissionString = keyof typeof FLAGS
export type PermissionsResolvable = number | Permissions | PermissionString | PermissionsResolvable[]

const FLAGS = {
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
    CHANGE_NICKNAME: 1 << 13,
    INVITE_OTHERS: 1 << 14
}

export declare interface Permissions {
    serialize(): Record<PermissionString, boolean>
    any(bit: PermissionsResolvable): boolean
    add(...bits: PermissionsResolvable[]): this
    missing(bits: PermissionsResolvable): PermissionString[]
    remove(...bits: PermissionsResolvable[]): this
    has(bit: PermissionsResolvable): boolean
    toArray(): PermissionString[]
    equals(bit: PermissionsResolvable): boolean
}

export class Permissions extends BitField {
    static FLAGS: typeof FLAGS
    constructor(...bits: PermissionsResolvable[]) {
        super(bits)
    }

    static async fetch(user: User | string, server?: Server | string | null, channel?: DMChannel | Group | TextChannel | Category): Promise<Permissions> {
        user = typeof user === 'string' ? await User.findOne({ id: user as ID }) as User : user

        const permissions = new Permissions()


        if (server) {
            let member: Member

            [member, server] = await Promise.all([
                Member.findOne({ id: typeof user === 'string' ? user : user.id }),
                typeof server === 'string' ? Server.findOne({ id: server }) : server,
            ]) as [Member, Server]

            if (member.id === server.owner.id) {
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
                    if (channel.owner.id === user.id) {
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
        if (checkAdmin && super.has(Permissions.FLAGS.ADMINISTRATOR)) return []
        return super.missing(bits) as PermissionString[]
    }

    any(bit: PermissionsResolvable, checkAdmin = true): boolean {
        if (checkAdmin && super.has(Permissions.FLAGS.ADMINISTRATOR)) return true
        return super.any(bit)
    }

    has(bit: PermissionsResolvable, checkAdmin = true): boolean {
        if (checkAdmin && super.has(Permissions.FLAGS.ADMINISTRATOR)) return true
        return super.has(bit)
    }
}

Permissions.FLAGS = FLAGS


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