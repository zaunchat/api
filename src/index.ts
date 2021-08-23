import app from './routes'
import config from '../config'
import * as database from './database'

const port = config('PORT')

declare global {
	const db: database.DBConnection
}

async function main(): Promise<void> {
	try {
		console.log('Connecting to database...')

		Object.defineProperty(globalThis, 'db', {
			value: await database.connect(config('DATABASE_URI'))
		})

		console.log('Connected to Database')

		app.listen(port, () => console.log(`App running on port: ${port}`))
	} catch (err) {
		console.error(err)
		process.exit(-1)
	}
}

main()
