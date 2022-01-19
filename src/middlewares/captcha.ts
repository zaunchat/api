import { Request, Response, NextFunction } from '@tinyhttp/app'
import { fetch } from '../utils'
import { HTTPError } from '../errors'
import config from '../config'

export const captcha = () => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
  const key = req.body.captcha_key

  if (!key) {
    throw new HTTPError('FAILED_CAPTCHA')
  }

  const payload = {
    secret: config.captcha.token,
    response: key,
    sitekey: config.captcha.key
  }

  const response = await fetch('https://hcaptcha.com/siteverify', {
    method: 'POST',
    body: payload,
  }).then((res) => res.json())

  if (!response || !response.success) {
    throw new HTTPError('FAILED_CAPTCHA')
  }

  next()
}
