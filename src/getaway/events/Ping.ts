import { WSCodes } from '../Constants'
import { Client } from '../Client'
import { WebSocket } from 'ws'

export const Ping = async (client: Client, socket: WebSocket): Promise<void> => {
    await client.setHeartbeat(socket).send({ code: WSCodes.PONG }, socket)
}