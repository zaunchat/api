import { BitField } from './BitField'

export type BadgeString = keyof typeof FLAGS
export type BadgesResolvable = bigint | number | Badges | BadgeString | BadgesResolvable[]

export declare interface Badges {
  serialize(): Record<BadgeString, boolean>
  any(bit: BadgesResolvable): boolean
  add(...bits: BadgesResolvable[]): this
  missing(bits: BadgesResolvable): BadgeString[]
  remove(...bits: BadgesResolvable[]): this
  has(bit: BadgesResolvable): boolean
  toArray(): BadgeString[]
  equals(bit: BadgesResolvable): boolean
}

const FLAGS = {
  STAFF: 1n << 1n,
  DEVELOPER: 1n << 2n,
  SUPPORTER: 1n << 3n,
  TRANSLATOR: 1n << 4n
} as const

export class Badges extends BitField {
  static FLAGS: typeof FLAGS
  constructor(...bits: BadgesResolvable[]) {
    super(bits)
  }
}

Badges.FLAGS = FLAGS
