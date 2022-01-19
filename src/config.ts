import 'dotenv/config'
import env from 'env-var'

const config = {
  port: env.get('PORT').default(8080).asPortNumber(),
  database: {
    uri: env.get('DATABASE_URI').required().asUrlString(),
    redis: env.get('REDIS_URI').required().asUrlString()
  },
  smtp: {
    enabled: env.get('SMTP_ENABLED').default('true').asBool(),
    host: env.get('SMTP_HOST').asUrlString(),
    username: env.get('SMTP_USERNAME').asString(),
    password: env.get('SMTP_PASSWORD').asString()
  },
  captcha: {
    enabled: env.get('CAPTCHA_ENABLED').default('true').asBool(),
    key: env.get('CAPTCHA_KEY').asString(),
    token: env.get('CAPTCHA_TOKEN').asString()
  },
  limits: {
    user: {
      username: 32,
      servers: 100,
      groups: 50,
      friends: 1_000,
      blocked: 1_000,
      bots: 10
    },
    server: {
      name: 50,
      description: 1_000,
      roles: 200,
      channels: 200,
      emojis: 50,
      members: 10_000
    },
    member: {
      nickname: 32
    },
    message: {
      length: 2_000,
      attachments: 5,
      replies: 5,
      embeds: 1
    },
    group: {
      name: 50,
      members: 100,
      description: 1_000
    },
    channel: {
      name: 50,
      topic: 1000
    }
  },
  endpoints: {
    main: env.get('DOMAIN').default('https://itchat.world').asUrlString(),
    app: '',
    api: '',
    cdn: ''
  }
}

const [protocol, hostname] = config.endpoints.main.split(/:\/\//, 1)

for (const subdomain of ['app', 'api', 'cdn'] as const) {
  config.endpoints[subdomain] = `${protocol}://${subdomain}.${hostname}`
}

export default Object.freeze(config)
