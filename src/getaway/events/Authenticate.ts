import { Payload, WSCloseCodes, WSCodes } from '../Constants'
import { Socket } from '../Socket'
import { DMChannel, User, Server, TextChannel, PresenceStatus, Group, ChannelTypes } from '../../structures'
import { WSEvents } from '../../@types'


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
        deleted: false,
        verified: true
    }, {
        fields: ['_id', 'avatar', 'username', 'badges', 'email', 'relations', 'servers', 'sessions']
    }) : null


    if (!user?.sessions.some(session => session.token === auth.token)) {
        return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
    }

    user.presence.status = PresenceStatus.ONLINE
    socket.user_id = user._id

    socket.getaway.connections.set(user._id, socket)

    await socket.send({ code: WSCodes.AUTHENTICATED })


    const [users, servers, dms, groups, channels] = await Promise.all([
        User.find({
            _id: {
                $in: Array.from(user.relations.keys())
            },
            deleted: false
        }, {
            fields: ['_id', 'avatar', 'username', 'badges']
        }),
        Server.find({
            _id: {
                $in: user.servers
            },
            deleted: false
        }),
        DMChannel.find({
            type: ChannelTypes.DM,
            recipients: user._id,
            deleted: false
        }),
        Group.find({
            type: ChannelTypes.GROUP,
            recipients: user._id,
            deleted: false
        }),
        TextChannel.find({
            serverId: {
                $in: user.servers
            },
            deleted: false
        })
    ])


    const readyData: WSEvents['READY'] = {
        user,
        users,
        servers,
        channels: [...dms, ...groups, ...channels],
        members: [] // TODO: Fetch members?
    }

    await socket.send({
        code: WSCodes.READY,
        data: readyData
    })


    await socket.subscribe([
        user._id,
        user.servers,
        Array.from(user.relations.keys()),
        readyData.channels.map(c => c._id)
    ])
}