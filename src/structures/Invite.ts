import { Base } from '.'
import { nanoid } from 'nanoid'
import sql from '../database'

export interface CreateInviteOptions extends Options<Invite> {
  inviter_id: string
  channel_id: string
  server_id: string
}


export class Invite extends Base {
  readonly code = nanoid(8)
  uses = 0
  inviter_id!: string
  channel_id!: string
  server_id!: string

  static from(opts: CreateInviteOptions): Invite {
    return Object.assign(new Invite(), opts)
  }
}
