import config from '../config'
import server from './server'
import db from './database'

(async () => {
	try {
		console.log('Connecting to database...')

		await db.connect()

		console.log('Connected to Database')

		server.listen(config.port, () => console.log(`App running on port: ${config.port}`))
	} catch (err) {
		console.error(err)
		process.exit(-1)
	}
})()