import { OPCODES } from '../Getaway'
import { Socket } from '../Socket'

export const Ping = (socket: Socket): void => {
    socket.setHeartbeat().send({ code: OPCODES.PONG })
}