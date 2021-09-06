import { Payload, WSCloseCodes, WSCodes, WSEvents } from '../Constants'
import { Channel, User, Server, PresenceStatus, ChannelTypes } from '../../structures'
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
        fields: ['_id', 'avatar', 'username', 'badges', 'email', 'relations', 'servers', 'sessions']
    }) : null


    if (!user?.sessions.some(session => session.token === auth.token)) {
        return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
    }

    socket.user_id = user._id

    socket.getaway.connections.set(user._id, socket)

    await socket.send({ code: WSCodes.AUTHENTICATED })


    const [
        users,
        servers,
        channels
    ] = await Promise.all([
        User.find({
            _id: {
                $in: Array.from(user.relations.keys())
            }
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        }),
        Server.find({
            _id: {
                $in: user.servers
            },

        }),
        Channel.find({
            $or: [{
                type: ChannelTypes.DM,
                recipients: user._id
            }, {
                type: ChannelTypes.GROUP,
                recipients: user._id
            }, {
                serverId: {
                    $in: user.servers
                }
            }]
        })
    ])


    const readyData: WSEvents['READY'] = {
        user,
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
        user.servers,
        channels.map(c => c._id)
    ])

    if (!user.presence.ghostMode) await user.save({
        presence: {
            ghostMode: user.presence.ghostMode,
            status: PresenceStatus.ONLINE
        }
    })
}