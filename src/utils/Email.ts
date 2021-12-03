import { nanoid } from 'nanoid'
import { User } from '../structures'
import { createRedisConnection } from '../database/redis'
import { SMTPClient, Message } from 'emailjs'
import config from '../config'
import ms from 'ms'

const THREE_HOURS = ms('3 hours')
const EMAIL_MESSAGE_TEMPLATE = `Hello @%%USERNAME%%,

You're almost there! If you did not perform this action you can safely ignore this email.
Please verify your account here: %%LINK%%`


class Email {
  redis = createRedisConnection()
  client = new SMTPClient({
    host: config.smtp.host,
    user: config.smtp.username,
    password: config.smtp.password
  })

  async send(user: User): Promise<string> {
    const code = nanoid(64)
    const link = `${config.endpoints.main}/auth/verify/${user.id}/${code}`
    const message = new Message({
      from: 'noreply@itchat.world',
      to: user.email,
      subject: 'Verify your account',
      text: EMAIL_MESSAGE_TEMPLATE
        .replace('%%USERNAME%%', user.username)
        .replace('%%LINK%%', link),
    })

    await this.client.sendAsync(message)

    // Expires after three hours.
    await this.redis.set(user.id, code, 'PX', THREE_HOURS)

    return link
  }

  async verify(key: string, code: string): Promise<boolean> {
    const exists = await this.redis.get(key)

    if (exists && exists === code) {

      // Allow to use one time
      await this.redis.del(key)

      return true
    }

    return false
  }
}

export const email = new Email()
