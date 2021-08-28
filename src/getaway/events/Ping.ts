import { WSCodes } from '../Getaway'
import { Socket } from '../Socket'

export const Ping = async (socket: Socket): Promise<void> => {
    await socket.setHeartbeat().send({ code: WSCodes.PONG })
}