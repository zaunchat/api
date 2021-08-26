import config from '../config'
import server from './server'
import db from './database'

const port = config('PORT')

async function main(): Promise<void> {
	try {
		console.log('Connecting to database...')

		await db.connect(config('DATABASE_URI'))

		console.log('Connected to Database')

		server.listen(port, () => console.log(`App running on port: ${port}`))
	} catch (err) {
		console.error(err)
		process.exit(-1)
	}
}

main()
