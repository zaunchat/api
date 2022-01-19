import Server from './server'
import migrations from './database/migrations'
import config from './config'
import { logger } from './utils'

export const server = new Server()


try {
  logger.log('Initialling the server...')

  await server.init({
    origin: config.endpoints.main
  })

  logger.log('Initialling the database...')

  await migrations.run()
  await server.listen(config.port)

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