import { cpus } from 'os'
import cluster from 'cluster'
import config from '../config'
import server from './server'
import db from './database'


if (cluster.isPrimary) {
	const totalCPUs = cpus().length

	for (let i = 0; i < totalCPUs; i++) {
		cluster.fork()
	}

	cluster.on('exit', (worker) => {
		console.log(`worker ${worker.process.pid} died`)
		cluster.fork()
	})
} else {
	(async () => {
		console.log('Connecting to database...')

		await db.connect()

		console.log('Connected to Database')

		server.listen(config.port, () => console.log(`Server running on port: ${config.port}`))
	})().catch(err => {
		console.error(err)
		process.exit(-1)
	})
}



process
	.on('unhandledRejection', err => err && console.error(err))
	.on('uncaughtException', console.error)