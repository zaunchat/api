import { WSCodes } from '../Getaway'
import { Authenticate } from './Authenticate'
import { Ping } from './Ping'
export default {
    [WSCodes.AUTHENTICATE]: Authenticate,
    [WSCodes.PING]: Ping
}