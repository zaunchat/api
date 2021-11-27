import { Request, Response, NextFunction } from '@tinyhttp/app'
import { fetch } from '../utils'
import config from '../config'


interface CaptchaOptions {
  required: `/${string}`[]
}

export const captcha = (options: CaptchaOptions) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
  if (!config.captcha.enabled || !options.required.some((p) => req.path.includes(p))) {
    return next()
  }

  const key = req.body.captcha_key

  if (!key) {
    req.throw('FAILED_CAPTCHA')
  }

  const payload = {
    secret: config.captcha.token,
    response: key,
    sitekey: config.captcha.key
  }

  const response = await fetch('https://hcaptcha.com/siteverify', {
    method: 'POST',
    body: JSON.stringify(payload),
  }).then((res) => res.json())

  if (!response || !response.success) {
    req.throw('FAILED_CAPTCHA')
  }

  next()
}
