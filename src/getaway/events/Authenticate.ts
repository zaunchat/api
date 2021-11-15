import { Payload, WSCloseCodes, WSCodes, WSEvents } from '../Constants'
import { Channel, User } from '../../structures'
import { Socket } from '../Socket'


export const Authenticate = async (socket: Socket, data: Payload): Promise<void> => {
    if (socket.user_id && socket.getaway.connections.has(socket.user_id)) {
        return socket.close(WSCloseCodes.ALREADY_AUTHENTICATED)
    }

    const auth = (data.data ?? {}) as {
        type: 'user',
        token: string
    }

    const user = await User.fetchByToken(auth.token)

    if (!user) {
        return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
    }

    socket.user_id = user.id
    socket.getaway.connections.set(user.id, socket)

    await socket.send({ code: WSCodes.AUTHENTICATED })

    const servers = await user.fetchServers()
    const serverIDs = servers.map(s => s.id)

    const [
        users,
        channels
    ] = await Promise.all([
        user.fetchRelations(),
        Channel.find(`server_id IN ${serverIDs} OR recipients::jsonb ? ${user.id}`)
    ])

    const clientUser = {
        id: user.id,
        username: user.username,
        avatar: user.avatar,
        badges: user.badges
    } as User

    const readyData: WSEvents['READY'] = {
        user: clientUser,
        users,
        servers,
        channels
    }

    await socket.send({
        code: WSCodes.READY,
        data: readyData
    })

    await socket.subscribe(...[
        [user.id],
        [...user.relations.keys()],
        serverIDs,
        channels.map(c => c.id)
    ].flat(4) as ID[])
}