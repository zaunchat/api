import { Base, Presence, PresenceStatus } from '.'
import sql from '../database'
import { validator } from '../utils'


export const CreateBotSchema = validator.compile({
  username: 'string'
})

export interface CreateBotOptions extends Options<Bot> {
  username: string
  owner_id: string
}

export class Bot extends Base {
  username!: string
  owner_id!: string
  avatar: Nullable<string> = null
  presence: Presence = { status: PresenceStatus.OFFLINE }
  verified = false

  static from(opts: CreateBotOptions): Bot {
    return Object.assign(new Bot(), opts)
  }
}
