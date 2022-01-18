import { SMTPClient, Message } from 'emailjs'
import { nanoid } from 'nanoid'
import { User } from '@structures'
import { createRedisConnection } from '@database/redis'
import config from '@config'
import ms from 'ms'

const THREE_HOURS = ms('3 hours')
const EMAIL_MESSAGE_TEMPLATE = `Hello @%%USERNAME%%,

You're almost there! If you did not perform this action you can safely ignore this email.
Please verify your account by clicking on this link: %%LINK%%`


class Email {
  readonly pendingAccounts = createRedisConnection()
  readonly client = new SMTPClient({
    host: config.smtp.host,
    user: config.smtp.username,
    password: config.smtp.password
  })

  async send(user: User): Promise<string> {
    const code = nanoid(64)
    const link = `${config.endpoints.api}/auth/verify/${user.id}/${code}`
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
    await this.pendingAccounts.set(user.id, code, 'PX', THREE_HOURS)

    return link
  }

  async verify(key: string, code: string): Promise<boolean> {
    const exists = await this.pendingAccounts.get(key)

    if (exists === code) {

      // Allow to use one time
      await this.pendingAccounts.del(key)

      return true
    }

    return false
  }
}

export const email = new Email()
