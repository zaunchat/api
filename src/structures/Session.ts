import { Base } from '.'
import { nanoid } from 'nanoid'
import sql from '../database'

export interface CreateSessionOptions extends Options<Session> {
  user_id: string
}

interface DeviceInfo {
  name?: string
}

export class Session extends Base {
  readonly token = nanoid(64)
  readonly user_id!: string
  info!: DeviceInfo

  static from(opts: CreateSessionOptions): Session {
    return Object.assign(new Session(), opts)
  }
}