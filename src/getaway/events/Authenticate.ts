import { Payload, WSCloseCodes, OPCODES } from '../Getaway'
import { Socket } from '../Socket'
import db from '../../database'
import { User } from '../../structures'

export const Authenticate = async (socket: Socket, data: Payload): Promise<void> => {
    const auth = data.data as { 
        user_id: string,
        token: string
    }

    const user = await db.get(User).findOne({
        _id: auth.user_id,
        sessions: { token: auth.token },
        deleted: false
    })

    if (!user) {
        return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
    }

    socket.send({ code: OPCODES.AUTHENTICATED })

    // TODO: Send Ready event
}