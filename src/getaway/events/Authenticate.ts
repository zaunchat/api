import { Payload, WSCloseCodes, WSCodes, WSEvents } from '../Constants'
import { Channel, User, PresenceStatus, ChannelTypes } from '../../structures'
import { Socket } from '../Socket'


export const Authenticate = async (socket: Socket, data: Payload): Promise<void> => {
    if (socket.user_id && socket.getaway.connections.has(socket.user_id)) {
        return socket.close(WSCloseCodes.ALREADY_AUTHENTICATED)
    }

    const auth = (data.data ?? {}) as {
        userid: string,
        token: string
    }

    const user = auth.user_id && auth.token ? await User.findOne({
        id: auth.user_id,
        verified: true
    }, {
        fields: ['_id', 'avatar', 'username', 'badges', 'email', 'relations', 'servers', 'sessions'],
        populate: ['servers']
    }) : null

    if (!user?.sessions?.getItems().some(session => session.token === auth.token)) {
        return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
    }

    socket.user_id = user.id
    socket.getaway.connections.set(user.id, socket)

    await socket.send({ code: WSCodes.AUTHENTICATED })

    const servers = user.servers.getItems()
    const serverIDs = servers.map(s => s.id)

    const [
        users,
        channels
    ] = await Promise.all([
        User.find({
            id: {
                $in: Array.from(user.relations.keys())
            }
        }, { public: true }),
        Channel.find({
            $or: [{
                type: ChannelTypes.DM,
                recipients: user.id
            }, {
                type: ChannelTypes.GROUP,
                recipients: user.id
            }, {
                serverid: {
                    $in: serverIDs
                }
            }]
        })
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
    ].flat(4))

    if (!user.presence.ghost_mode) await user.save({
        presence: {
            ghost_mode: user.presence.ghost_mode,
            status: PresenceStatus.ONLINE
        }
    })
}