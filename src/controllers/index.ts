import { Controller } from './Controller'

export class PingController extends Controller('/ping') {
  'GET /'(): string {
    return `Pong! ${process.uptime()}`
  }
}

export * from './auth'
export * from './servers'
export * from './UserController'
export * from './MessageController'
export * from './ChannelController'
export * from './InviteController'
export * from './BotController'
