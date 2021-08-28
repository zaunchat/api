import { Payload, WSCloseCodes, WSCodes } from '../Getaway'
import { Socket } from '../Socket'
import db from '../../database'
import { DMChannel, User, Server } from '../../structures'
import { WSEvents } from '../../@types'

export const Authenticate = async (socket: Socket, data: Payload): Promise<void> => {
    const auth = data.data as {
        user_id: string,
        token: string
    }

    const user = await db.get(User).findOne({
        _id: auth.user_id,
        sessions: { token: auth.token },
        deleted: false
    }, {
        fields: ['_id', 'avatar', 'username', 'badges', 'email', 'relations', 'servers']
    })

    if (!user) {
        return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
    }

    await socket.send({ code: WSCodes.AUTHENTICATED })

    const channels = await db.get(DMChannel).find({
        recipients: user._id,
        deleted: false
    })

    const users = await db.get(User).find({
        _id: {
            $in: user.relations.map(({ id }) => id)
        },
        deleted: false
    }, {
        fields: ['_id', 'avatar', 'username', 'badges']
    })

    const servers = await db.get(Server).find({
        _id: {
            $in: user.servers
        },
        deleted: false
    })

    const readyData: WSEvents['READY'] = {
        user,
        channels,
        users,
        servers,
        members: []
    }

    await socket.send({
        code: WSCodes.READY,
        data: readyData
    })
}