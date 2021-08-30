import fetch from 'node-fetch'
import config from '../../config'

export class Captcha extends null {
    static async check(response: string): Promise<boolean> {
        const payload = {
            secret: config.captcha.token,
            response,
            sitekey: config.captcha.key
        }

        const res = await fetch('https://hcaptcha.com/siteverify', {
            method: 'POST',
            body: JSON.stringify(payload),
        }).then((res) => res.json()).catch(() => false)

        return Boolean(res?.success)
    }
}