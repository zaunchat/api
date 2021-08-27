import { WSCodes } from '../Getaway'
import { Socket } from '../Socket'

export const Ping = (socket: Socket): void => {
    socket.setHeartbeat().send({ code: WSCodes.PONG })
}