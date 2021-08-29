import * as web from 'express-decorators'

@web.basePath('/servers/:serverId/channels')
export class ServerChannelController {}