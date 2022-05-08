import { App } from '@tinyhttp/app'
import { AccountController, SessionController } from './auth'
import { BotController } from './BotController'
import { ChannelController, MessageController } from './channels'
import { InviteController } from './InviteController'
import { PingController } from './PingController'
import { ServerChannelController, ServerController, ServerInviteController, ServerMemberController, ServerRoleController} from './servers'
import { UserController } from './UserController'


export const mount = (app: App) => {
  // Auth
  new AccountController().mount(app, '/auth/accounts')
  new SessionController().mount(app, '/auth/sessions')

  // Servers
  new ServerController().mount(app, '/servers')
  new ServerChannelController().mount(app, '/channels/:server_id')
  new ServerInviteController().mount(app, '/servers/:server_id/invites')
  new ServerMemberController().mount(app, '/servers/:server_id/members')
  new ServerRoleController().mount(app, '/servers/:server_id/roles')

  // Channels
  new ChannelController().mount(app, '/channels/@me')
  new MessageController().mount(app, '/channels/:channel_id/messages')

  // Other
  new InviteController().mount(app, '/invites')
  new UserController().mount(app, '/users')
  new PingController().mount(app, '/ping')
  new BotController().mount(app, '/bots')
}