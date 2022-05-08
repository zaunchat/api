import { Base, QueryBuilder, WhereCondition } from '.'
import { DEFAULT_PERMISSION_DM, validator } from '../utils'
import { APIErrors, HTTPError } from '../errors'
import sql from '../database'
import config from '../config'

export enum ChannelTypes {
  DM,
  TEXT,
  VOICE,
  CATEGORY,
  GROUP
}

export enum OverwriteTypes {
  ROLE,
  MEMBER
}

export interface Overwrite {
  id: string
  type: OverwriteTypes
  allow: bigint
  deny: bigint
}

interface CreateGroupChannelOptions extends Options<GroupChannel> {
  type: ChannelTypes.GROUP
  name: string
  recipients: string[]
  owner_id: string
}

interface CreateTextChannelOptions extends Options<TextChannel> {
  type: ChannelTypes.TEXT
  name: string
  server_id: string
}

interface CreateCategoryChannelOptions extends Options<CategoryChannel> {
  type: ChannelTypes.CATEGORY
  name: string
  server_id: string
}


interface CreateDMChannelOptions extends Options<DMChannel> {
  type: ChannelTypes.DM
  recipients: string[]
}

export const CreateServerChannelSchema = validator.compile({
  type: {
    type: 'enum',
    values: Object.keys(ChannelTypes).filter(k => !isNaN(+k))
  },
  name: `string|min:1|max:${config.limits.channel.name}`,
  topic: `string|min:1|max:${config.limits.channel.topic}|optional`
})


export const CreateGroupSchema = validator.compile({
  name: `string|min:1|max:${config.limits.group.name}`
})

export type AnyChannel = TextChannel | DMChannel | CategoryChannel | GroupChannel

export type ServerChannel = TextChannel | CategoryChannel /* | VoiceChannel */

export class Channel extends Base {
  readonly type!: ChannelTypes

  static async findOne<T = AnyChannel>(
    where: WhereCondition<T> | ((query: QueryBuilder<T>) => unknown)
  ): Promise<T> {
    const query = new QueryBuilder<T>({ table: this.tableName, limit: 1 })

    typeof where === 'function' ? where(query) : query.where(where)

    const [item] = await sql.unsafe(query.text, query.values) as [T?]

    if (!item) {
      const tag = `UNKNOWN_${this.name.toUpperCase()}` as keyof typeof APIErrors

      if (!(tag in APIErrors)) {
        throw new Error('Unhandled type')
      }

      throw new HTTPError(tag)
    }

    return item
  }

  static async find<T = AnyChannel>(
    where: WhereCondition<T> | ((query: QueryBuilder<T>) => unknown),
    limit = 100
  ): Promise<T[]> {
    const query = new QueryBuilder<T>({ table: this.tableName, limit })

    typeof where === 'function' ? where(query) : query.where(where)

    return sql.unsafe(query.text, query.values) as Promise<T[]>
  }

  isText(): this is TextChannel {
    return this.type === ChannelTypes.TEXT
  }

  isCategory(): this is CategoryChannel {
    return this.type === ChannelTypes.CATEGORY
  }

  isDM(): this is DMChannel {
    return this.type === ChannelTypes.DM
  }

  isGroup(): this is GroupChannel {
    return this.type === ChannelTypes.GROUP
  }

  isVoice(): this is VoiceChannel {
    return this.type === ChannelTypes.VOICE
  }

  inServer(): this is ServerChannel {
    return 'server_id' in this
  }

  static from(opts: CreateTextChannelOptions): TextChannel
  static from(opts: CreateDMChannelOptions): DMChannel
  static from(opts: CreateCategoryChannelOptions): CategoryChannel
  static from(opts: CreateGroupChannelOptions): GroupChannel
  /* static from(opts: CreateVoiceChannelOptions): VoiceChannel */
  static from(opts: { type: ChannelTypes } & Partial<AnyChannel>): AnyChannel {
    let channel: AnyChannel

    switch (opts.type) {
      case ChannelTypes.TEXT: channel = new TextChannel()
        break
      case ChannelTypes.DM: channel = new DMChannel()
        break
      case ChannelTypes.GROUP: channel = new GroupChannel()
        break
      case ChannelTypes.CATEGORY: channel = new CategoryChannel()
        break
      // case ChannelTypes.VOICE: channel = new VoiceChannel()
      //   break
      default: throw new Error('Unknown channel type')
    }

    return Object.assign(channel, opts)
  }
}



export class DMChannel extends Channel {
  readonly type = ChannelTypes.DM
  recipients: string[] = []
}

export class GroupChannel extends Channel {
  readonly type = ChannelTypes.GROUP
  name!: string
  owner_id!: string
  permissions = DEFAULT_PERMISSION_DM
  recipients: string[] = []
}

export class TextChannel extends Channel {
  readonly type = ChannelTypes.TEXT
  name!: string
  server_id!: string
  overwrites: Overwrite[] = []
  parent_id: Nullable<string> = null
}

export class CategoryChannel extends Channel {
  readonly type = ChannelTypes.CATEGORY
  name!: string
  server_id!: string
  overwrites: Overwrite[] = []
}


export class VoiceChannel extends Channel {
  readonly type = ChannelTypes.VOICE
  name!: string
  // TODO: Add other stuff...
}