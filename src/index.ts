import Server from './server'
import config from './config'
import migrations from './database/migrations'
import { logger } from './utils'
import { User } from './structures'


export const server = new Server({
  port: config.port,
  limits: {
    global: '20/5s',
    'auth/accounts/login': '3/24h --ip',
    'auth/accounts/register': '3/24h --ip',
    'auth/accounts/verify': '2/24h --ip',
    servers: '5/5s',
    channels: '5/5s',
    users: '5/5s'
  }
})

try {
  logger.log('Initialling the server...')

  await server.init()

  logger.log('Initialling the database...')

  await migrations.run()
  await server.listen()


  const u = User.from({
    username: 'maestroo!',
    password: 'great-password',
    email: 'email@gmail.com'
  })

  logger.log('Saving user..')
  await u.save()


  logger.log(await User.find({ id: u.id }))

  logger.log('Server running on port:', config.port)
} catch (err) {

  logger
    .error('Failed to Init the server....')
    .error(err)
    .error('Exiting...')

  process.exit(-1)
}

process
  .on('unhandledRejection', logger.error)
  .on('uncaughtException', logger.error)
