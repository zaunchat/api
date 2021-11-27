import { WSCodes } from '../Constants'
import { Authenticate } from './Authenticate'
import { Ping } from './Ping'

export default {
  [WSCodes.AUTHENTICATE]: Authenticate,
  [WSCodes.PING]: Ping
}
