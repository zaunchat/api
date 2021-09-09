import { Payload, WSCloseCodes, WSCodes, WSEvents } from '../Constants'
import { Channel, User, PresenceStatus, ChannelTypes } from '../../structures'
import { Socket } from '../Socket'


export const Authenticate = async (socket: Socket, data: Payload): Promise<void> => {
    if (socket.user_id && socket.getaway.connections.has(socket.user_id)) {
        return socket.close(WSCloseCodes.ALREADY_AUTHENTICATED)
    }

    const auth = (data.data ?? {}) as {
        user_id: string,
        token: string
    }

    const user = auth.user_id && auth.token ? await User.findOne({
        _id: auth.user_id,
        verified: true
    }, {
        fields: ['_id', 'avatar', 'username', 'badges', 'email', 'relations', 'servers', 'sessions'],
        populate: ['servers']
    }) : null


    if (!user?.sessions.some(session => session.token === auth.token)) {
        return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
    }

    socket.user_id = user._id
    socket.getaway.connections.set(user._id, socket)

    await socket.send({ code: WSCodes.AUTHENTICATED })

    const servers = user.servers.getItems()
    const serverIDs = servers.map(s => s._id)

    const [
        users,
        channels
    ] = await Promise.all([
        User.find({
            _id: {
                $in: Array.from(user.relations.keys())
            }
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        }),
        Channel.find({
            $or: [{
                type: ChannelTypes.DM,
                recipients: user._id
            }, {
                type: ChannelTypes.GROUP,
                recipients: user._id
            }, {
                server_id: {
                    $in: serverIDs
                }
            }]
        })
    ])

    const clientUser = {
        _id: user._id,
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


    await socket.subscribe([
        [user._id],
        [...user.relations.keys()],
        serverIDs,
        channels.map(c => c._id)
    ])

    if (!user.presence.ghost_mode) await user.save({
        presence: {
            ghost_mode: user.presence.ghost_mode,
            status: PresenceStatus.ONLINE
        }
    })
}