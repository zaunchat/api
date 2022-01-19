import { Payload, WSCloseCodes, WSCodes, WSEvents } from '../Constants'
import { Channel, User } from '../../structures'
import { Client } from '../Client'
import { Permissions, validator } from '../../utils'
import sql from '../../database'
import { getaway } from '../../getaway'
import { WebSocket } from 'ws'

const isValidAuth = validator.compile({
  type: {
    type: 'enum',
    values: ['user', 'bot']
  },
  token: 'string'
})

export const Authenticate = async (client: Client, socket: WebSocket, payload: Payload): Promise<void> => {
  if (client.authenticated) {
    return socket.close(WSCloseCodes.ALREADY_AUTHENTICATED)
  }

  const auth = payload.data as {
    type: 'user' | 'bot'
    token: string
  }

  if (isValidAuth(auth) !== true) {
    return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
  }

  const user = await User.fetchByToken(auth.token)

  if (!user) {
    return socket.close(WSCloseCodes.AUTHENTICATED_FAILED)
  }

  await client.send({ code: WSCodes.AUTHENTICATED }, socket)

  const otherClient = getaway.clients.get(user.id)

  if (otherClient) {
    otherClient.connections.add(socket)
    client.connections.delete(socket)
    return
  }

  getaway.clients.set(user.id, client)

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

  await client.subscribe([
    [user.id],
    Object.keys(user.relations),
    extractIDs(servers),
    extractIDs(channels)
  ].flat(4))

  const promises: Promise<Permissions>[] = []

  for (const server of servers) {
    promises.push(Permissions.fetch({ user, server }))
  }

  for (const channel of channels) {
    promises.push(Permissions.fetch({ user, channel }))
  }

  for (const permission of await Promise.all(promises)) {
    client.permissions.set(permission.target_id, permission)
  }

  Object.defineProperty(client, 'user_id', {
    value: user.id
  })

  const readyData: WSEvents['READY'] = {
    user,
    users,
    servers,
    channels
  }

  await client.send({
    code: WSCodes.READY,
    data: readyData
  })
}



function extractIDs(x: { id: string }[]): string[] {
  return x.map(({ id }) => id)
}
