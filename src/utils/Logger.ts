import colors from 'kleur'
import { format } from 'node:util'

const time = new Date()

class Logger {
  private static write = (text: string) => process.stdout.write(text + '\n')

  private _log(data: unknown[], tag: string): this {
    time.setTime(Date.now())
    Logger.write(`[${time.toTimeString().slice(0, 8)}] [${tag}]: ${format(...data)}`)
    return this
  }

  log(...data: unknown[]): this {
    return this._log(data, colors.green('LOG'))
  }

  error(...data: unknown[]): this {
    return this._log(data, colors.red('ERROR'))
  }

  info(...data: unknown[]): this {
    return this._log(data, colors.blue('INFO'))
  }

  warn(...data: unknown[]): this {
    return this._log(data, colors.yellow('WARN'))
  }
}


export const logger = new Logger()
