import { Payload, WSCloseCodes, WSCodes, WSEvents } from '../Constants'
import { Channel, ChannelTypes, User } from '../../structures'
import { Socket } from '../Socket'
import { Permissions } from '../../utils'
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


  const [
    servers,
    users,
    channels
  ] = await Promise.all([
    user.fetchServers(),
    user.fetchRelations(),
    sql<Channel[]>`
    SELECT * FROM channels
    WHERE server_id IN ( SELECT server_id FROM members WHERE id = ${user.id} )
    OR recipients ? ${user.id}`
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
    extractIDs(servers),
    extractIDs(channels)
  ].flat(4))

  const promises: Promise<Permissions>[] = [], ids: string[] = []

  for (const server of servers) {
    promises.push(Permissions.fetch({ user, server }))
    ids.push(server.id)
  }

  for (const channel of channels) {
    if (channel.type === ChannelTypes.DM) continue
    promises.push(Permissions.fetch({ user, channel }))
    ids.push(channel.id)
  }

  const result = await Promise.all(promises)

  for (let i = 0; i < result.length; i++) {
    socket.permissions.set(ids[i], result[i])
  }

  socket.isPermissionsCached = true
}



function extractIDs(x: { id: string }[]): string[] {
  return x.map(({ id }) => id)
}
