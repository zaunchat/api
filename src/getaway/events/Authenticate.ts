import { Payload, WSCloseCodes, WSCodes, WSEvents } from '../Constants'
import { Channel, User } from '../../structures'
import { Socket } from '../Socket'
import sql from '../../database'

export const Authenticate = async (socket: Socket, data: Payload): Promise<void> => {
  if (socket.user_id) {
    return socket.close(WSCloseCodes.ALREADY_AUTHENTICATED)
  }

  const auth = (data.data ?? {}) as {
    type: 'user',
    token: string
  }

  if (!auth.token || !auth.type) { // Ignore kidding..
    return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
  }

  const user = await User.fetchByToken(auth.token)

  if (!user) {
    return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
  }

  socket.user_id = user.id
  socket.getaway.connections.set(user.id, socket)

  await socket.send({ code: WSCodes.AUTHENTICATED })

  const servers = await user.fetchServers()
  const serverIDs = extractIDs(servers)

  const [
    users,
    channels
  ] = await Promise.all([
    user.fetchRelations(),
    sql<Channel[]>`SELECT * FROM ${sql(Channel.tableName)} WHERE server_id IN (${sql(serverIDs)}) OR recipients ? ${user.id}`
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

  await socket.subscribe([
    [user.id],
    extractIDs(user.relations),
    serverIDs,
    extractIDs(channels)
  ].flat(4) as ID[])
}



function extractIDs(x: { id: string }[]): string[] {
  return x.map(({ id }) => id)
}
