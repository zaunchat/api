import Server from './server'
import config from './config'
import migrations from './database/migrations'
import { logger } from './utils'


export const server = new Server({
  port: config.port,
  limits: {
    global: '20/5s',
    'auth/login': '3/24h --ip',
    'auth/register': '3/24h --ip',
    'auth/verify': '2/24h --ip',
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
