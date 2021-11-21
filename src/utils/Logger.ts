import kleur from 'kleur'

const time = () => new Date().toTimeString().slice(0, 8)

const color = (mode: 'error' | 'info' | 'warn' | 'log'): string => {
  switch (mode) {
    case 'error': return kleur.red('ERROR')
    case 'warn': return kleur.yellow('WARN')
    case 'info': return kleur.blue('INFO')
    case 'log': return kleur.green('LOG')
  }
}

class Logger {
  private _log(data: unknown[], mode: string): this {
    console.log(`[${time()}] [${color(mode as 'log')}]:`, ...data)
    return this
  }

  log(...data: unknown[]): this {
    return this._log(data, 'log')
  }

  error(...data: unknown[]): this {
    return this._log(data, 'red')
  }

  info(...data: unknown[]): this {
    return this._log(data, 'blue')
  }

  warn(...data: unknown[]): this {
    return this._log(data, 'yellow')
  }
}


export const logger = new Logger()
