import { Controller } from './Controller'

export class PingController extends Controller {
    'GET /'(): string {
      return `Pong! ${process.uptime()}`
    }
}