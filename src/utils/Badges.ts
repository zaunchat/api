import { BitField } from './BitField'

export type BadgeString = keyof typeof FLAGS
export type BadgesResolvable = number | Badges | BadgeString | BadgesResolvable[]

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
	STAFF: 1 << 1,
	DEVELOPER: 1 << 2,
	SUPPORTER: 1 << 3,
	TRANSLATOR: 1 << 4
}

export class Badges extends BitField {
	static FLAGS: typeof FLAGS
	constructor(...bits: BadgesResolvable[]) {
		super(bits)
	}
}

Badges.FLAGS = FLAGS