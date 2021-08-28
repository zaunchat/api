import { App } from '@tinyhttp/app'
import { middlewares } from './utils'
import { register } from 'express-decorators'
import * as Controllers from './controllers'
import { Getaway } from './getaway'


export const getaway = new Getaway()
export const server = new App()
    .use(middlewares.json())
    .use(middlewares.auth())
    .use('/ws', middlewares.ws(getaway.server))


for (const Controller of Object.values(Controllers)) {
    register(server, new Controller())
}


export default server