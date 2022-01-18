import type { Client } from '../Client'
import type { WebSocket } from 'ws'
import { Payload, WSCodes } from '../Constants'
import { Authenticate } from './Authenticate'
import { Ping } from './Ping'

type Event = (client: Client, socket: WebSocket, payload: Payload) => Promise<unknown | void>

const events = {
  [WSCodes.AUTHENTICATE]: Authenticate,
  [WSCodes.PING]: Ping
}

export default events as unknown as Record<WSCodes, Event>