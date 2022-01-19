import { Badges, is } from '.'
import { Member, Server, User, Channel, AnyChannel, OverwriteTypes, CategoryChannel, RelationshipStatus } from '../structures'
import { BitField } from './BitField'
import { Request } from '@tinyhttp/app'
import { Context } from '../controllers/Controller'

export type PermissionString = keyof typeof FLAGS
export type PermissionsResolvable = bigint | number | Permissions | PermissionString | PermissionsResolvable[]

export interface FetchPermissionsOptions {
  user: User
  channel?: Channel | string
  server?: Server | string
}

const FLAGS = {
  // Admin
  ADMINISTRATOR: 1n << 0n,


  // Channel
  VIEW_CHANNEL: 1n << 1n,
  SEND_MESSAGES: 1n << 2n,
  READ_MESSAGE_HISTORY: 1n << 3n,
  EMBED_LINKS: 1n << 4n,
  UPLOAD_FILES: 1n << 5n,


  // Manage
  MANAGE_SERVER: 1n << 6n,
  MANAGE_CHANNELS: 1n << 7n,
  MANAGE_MESSAGES: 1n << 8n,
  MANAGE_ROLES: 1n << 9n,
  MANAGE_NICKNAMES: 1n << 10n,
  BAN_MEMBERS: 1n << 11n,
  KICK_MEMBERS: 1n << 12n,


  // Member
  CHANGE_NICKNAME: 1n << 13n,
  INVITE_OTHERS: 1n << 14n
} as const

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
  target_id!: string

  constructor(...bits: PermissionsResolvable[]) {
    super(bits)
  }

  static async from(request: Request | Context): Promise<Permissions> {
    if (request instanceof Context) request = request.request
    if (typeof request.permissions !== 'undefined') return request.permissions

    const result = await Permissions.fetch({
      user: request.user,
      server: request.params.server_id,
      channel: request.params.channel_id
    })

    return request.permissions = result
  }

  static async fetch({ user, server, channel }: FetchPermissionsOptions): Promise<Permissions> {
    if (is.snowflake(server)) server = await Server.findOne({ id: server })
    if (is.snowflake(channel)) channel = await Channel.findOne<AnyChannel>({ id: channel })
    if (!server && channel?.inServer()) server = await Server.findOne({ id: channel.server_id })

    let member: Member | null = null

    const permissions = new Permissions()

    const admin = () => permissions.set(Permissions.FLAGS.ADMINISTRATOR)

    permissions.target_id = user.id

    if (server) permissions.target_id = server.id
    if (channel) permissions.target_id = channel.id

    // Yes. we do that.
    if (user.badges !== 0n && new Badges(user.badges).has(Badges.FLAGS.STAFF)) {
      return admin()
    }


    if (server) {
      if (user.id === server.owner_id) return admin()

      permissions.add(server.permissions) // Add default @everyone permissions.

      member = await user.member(server)

      for (const role of await server.fetchRoles()) {
        if (member.roles.includes(role.id)) permissions.add(role.permissions)
      }

      // We don't need any other checks if they has the "ADMINISTRATOR" permission.
      if (permissions.has(Permissions.FLAGS.ADMINISTRATOR, false)) return permissions
    }

    if (channel?.isGroup() && channel.recipients.includes(user.id)) {
      if (channel.owner_id === user.id) return admin()
      permissions.add(channel.permissions)
    }

    if (channel?.isDM() && channel.recipients.includes(user.id)) {
      permissions.add(DEFAULT_PERMISSION_DM)

      const isBlocked = channel.recipients.some(id => {
        return (
          user.relations[id] === RelationshipStatus.BLOCKED ||
          user.relations[id] === RelationshipStatus.BLOCKED_BY_OTHER
        )
      })

      if (isBlocked) permissions.remove(Permissions.FLAGS.SEND_MESSAGES)
    }

    if (channel?.isText() || channel?.isCategory() /* | channel?.isVoice() */) {
      member ??= await Member.findOne({ id: user.id })

      const overwrites = [...channel.overwrites]

      if (!channel.isCategory() && channel.parent_id) {
        const parent = await Channel.findOne<CategoryChannel>({ id: channel.parent_id })
        overwrites.push(...parent.overwrites)
      }

      for (const overwrite of overwrites) {
        if (
          (overwrite.type === OverwriteTypes.MEMBER && overwrite.id === user.id) ||
          (overwrite.type === OverwriteTypes.ROLE && member.roles.includes(overwrite.id))
        ) {
          permissions.add(overwrite.allow).remove(overwrite.deny)
        }
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
