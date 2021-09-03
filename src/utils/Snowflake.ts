import cluster from 'cluster'

export class SnowflakeUtil extends null {
  static readonly EPOCH = new Date('2021-01-01').getTime()
  static INCREMENT = 0
  static processId = BigInt(process.pid % 31)
  static workerId = BigInt((cluster.worker?.id || 0) % 31)

  static is(id: unknown): id is Snowflake {
    if (typeof id !== 'string') return false
    return /^\d{17,19}$/.test(id)
  }


  static idToBinary(num: string): string {
    let bin = ''
    let high = parseInt(num.slice(0, -10)) || 0
    let low = parseInt(num.slice(-10))
    while (low > 0 || high > 0) {
      bin = String(low & 1) + bin
      low = Math.floor(low / 2)
      if (high > 0) {
        low += 5000000000 * (high % 2)
        high = Math.floor(high / 2)
      }
    }
    return bin
  }


  static binaryToId(num: string): Snowflake {
    let dec = ''

    while (num.length > 50) {
      const high = parseInt(num.slice(0, -32), 2)
      const low = parseInt((high % 10).toString(2) + num.slice(-32), 2)
      dec = (low % 10).toString() + dec
      num = Math.floor(high / 10).toString(2) + Math.floor(low / 10).toString(2).padStart(32, '0')
    }

    let number = parseInt(num, 2)

    while (number > 0) {
      dec = (number % 10).toString() + dec
      number = Math.floor(number / 10)
    }

    return dec as Snowflake
  }

  static generate(now = Date.now()): Snowflake {
    if (this.INCREMENT >= 4095) this.INCREMENT = 0
    const time = BigInt(now - this.EPOCH) << 22n
    const workerId = this.workerId << 17n
    const processId = this.processId << 12n
    const increment = BigInt(this.INCREMENT++)
    return (time | workerId | processId | increment).toString() as Snowflake
  }


  static deconstruct(snowflake: string): {
    timestamp: number
    date: Date
    workerId: number
    processId: number
    increment: number
    binary: string
  } {
    const BINARY = this.idToBinary(snowflake).padStart(64, '0')
    return {
      timestamp: parseInt(BINARY.substring(0, 42), 2) + this.EPOCH,
      get date() {
        return new Date(this.timestamp)
      },
      workerId: parseInt(BINARY.substring(42, 47), 2),
      processId: parseInt(BINARY.substring(47, 52), 2),
      increment: parseInt(BINARY.substring(52, 64), 2),
      binary: BINARY,
    }
  }
}
