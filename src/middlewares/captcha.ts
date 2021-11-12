import { Request, Response, NextFunction } from '@tinyhttp/app'
import { HTTPError } from '../errors'
import config from '../config'

interface CaptchaOptions {
    required: `/${string}`[]
}

export const captcha = (options: CaptchaOptions) => async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    if (config.captcha.enabled && options.required.some((p) => req.path.includes(p))) {
        if (!req.body.captcha_key) {
            throw new HTTPError('FAILED_CAPTCHA')
        }

        const payload = {
            secret: config.captcha.token,
            response: req.body.captcha_key,
            sitekey: config.captcha.key
        }

        const res = await fetch('https://hcaptcha.com/siteverify', {
            method: 'POST',
            body: JSON.stringify(payload),
        }).then((res) => res.json()).catch(() => false)


        if (!res?.success) {
            throw new HTTPError('FAILED_CAPTCHA')
        }
    }

    next()
}