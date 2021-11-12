import postgres from 'postgres'
import config from '../config'

export default postgres(config.database.uri)