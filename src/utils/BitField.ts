export const DEFAULT_BIT = 0n

export type BitFieldResolvable = bigint | number | BitField | string | BitFieldResolvable[]

export declare interface BitField {
  constructor: typeof BitField
}

export class BitField {
  static FLAGS: Record<string, bigint>
  bitfield = DEFAULT_BIT

  constructor(bits: BitFieldResolvable) {
    this.bitfield = this.constructor.resolve(bits)
  }

  missing(bits: BitFieldResolvable): string[] {
    return new this.constructor(bits).remove(this).toArray()
  }

  any(bit: BitFieldResolvable): boolean {
    return (this.bitfield & this.constructor.resolve(bit)) !== DEFAULT_BIT
  }

  has(bit: BitFieldResolvable): boolean {
    return (this.bitfield & this.constructor.resolve(bit)) === bit
  }

  set(bits: bigint): this {
    this.bitfield = bits
    return this
  }

  add(...bits: BitFieldResolvable[]): this {
    let total = 0n

    for (const bit of bits) {
      total |= this.constructor.resolve(bit)
    }

    if (Object.isFrozen(this)) return new this.constructor(this.bitfield | total) as this

    this.bitfield |= total

    return this
  }

  remove(...bits: BitFieldResolvable[]): this {
    let total = 0n

    for (const bit of bits) {
      total |= this.constructor.resolve(bit)
    }

    if (Object.isFrozen(this)) return new this.constructor(this.bitfield & ~total) as this

    this.bitfield &= ~total

    return this
  }

  freeze(): Readonly<this> {
    return Object.freeze(this)
  }

  valueOf(): bigint {
    return this.bitfield
  }

  serialize(): Record<string, boolean> {
    const serialized: Record<string, boolean> = {}
    for (const [flag, bit] of Object.entries(this.constructor.FLAGS)) serialized[flag] = this.has(bit)
    return serialized
  }

  toArray(): string[] {
    const flags = Object.keys(this.constructor.FLAGS)
    return flags.filter(bit => this.has(bit))
  }

  equals(bit: BitFieldResolvable): boolean {
    return this.bitfield === this.constructor.resolve(bit)
  }

  *[Symbol.iterator](): Iterable<string> {
    yield* this.toArray()
  }

  static resolve(bit: BitFieldResolvable): bigint {
    if (typeof bit === 'bigint') return bit
    if (typeof bit === 'number') return BigInt(bit)
    if (Array.isArray(bit)) return bit.map(p => this.resolve(p)).reduce((prev, p) => prev | p, DEFAULT_BIT)
    if (bit instanceof BitField) return bit.bitfield
    if (typeof this.FLAGS[bit] !== 'undefined') return this.FLAGS[bit]
    throw new Error('Invalid Bit')
  }
}


BitField.FLAGS = {}
