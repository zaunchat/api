import postgres from 'postgres'
import url from 'node:url'
import { join } from 'node:path'
import config from '../config'
import {
  Invite,
  Member,
  User,
  Message,
  Session,
  Channel,
  Role,
  Server,
  Bot
} from '../structures'

const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
const noop = () => { }

const sql = postgres(config.database.uri, {
  types: {
    number: {
      to: 0,
      from: [21, 23, 26, 700, 701],
      serialize: value => {
        if (value !== null && typeof value === 'object') return JSON.stringify(value)
        return String(value)
      },
      parse: value => Number(value)
    }
  },
  transform: {
    row: (x: any) => {
      if ('username' in x) return User.from(x)
      if ('code' in x) return Invite.from(x)
      if ('nickname' in x) return Member.from(x)
      if ('embeds' in x) return Message.from(x)
      if ('token' in x) return Session.from(x)
      if ('type' in x) return Channel.from(x)
      if ('hoist' in x) return Role.from(x)
      if ('owner_id' in x && 'presence' in x) return Bot.from(x)
      if ('owner_id' in x) return Server.from(x)
      return x
    }
  },
  onnotice: noop
})

const DATE = '2022-4'

export const migrate = async () => {
  await sql.file(join(__dirname, `../../assets/migrations/${DATE}.sql`))
}

export default sql
