import { OPCODES } from '../Getaway'
import { Authenticate } from './Authenticate'
import { Ping } from './Ping'
export default {
    [OPCODES.AUTHENTICATE]: Authenticate,
    [OPCODES.PING]: Ping
}