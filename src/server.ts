import { App } from '@tinyhttp/app'
import { middlewares } from './utils'
import { register } from 'express-decorators'
import * as Controllers from './controllers'

const server = new App()
    .use(middlewares.json())
    .use(middlewares.auth())


for (const Controller of Object.values(Controllers)) {
    register(server, new Controller())
}

export default server