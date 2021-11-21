const time = () => new Date().toTimeString().slice(0, 8)

class Logger {
  private _log(data: unknown[], _color = 'green'): this {
    console.log(`[${time()}]:`, ...data)
    return this
  }

  log(...data: unknown[]): this {
    return this._log(data)
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
