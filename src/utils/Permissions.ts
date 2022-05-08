import { is } from '.'
import { Member, Server, User, Channel, AnyChannel, OverwriteTypes, CategoryChannel, RelationshipStatus } from '../structures'
import { Request } from '@tinyhttp/app'
import { Context } from '../controllers/Controller'
import { Permissions as BasicPermissions, Badges, DEFAULT_PERMISSION_DM } from '@itchatt/utils'


export interface FetchPermissionsOptions {
  user: User
  channel?: Channel | string
  server?: Server | string
}


export class Permissions extends BasicPermissions {
  target_id!: string

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
      if (permissions.has(Permissions.FLAGS.ADMINISTRATOR)) return permissions
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
}