const DEFAULT_BIT = 0

type Constructor<T> = new (...args: unknown[]) => T

export type BitFieldResolvable = number | BitField | string | BitFieldResolvable[]

export class BitField {
	static FLAGS: Record<string, number>
	bitfield = DEFAULT_BIT

	constructor(bits: BitFieldResolvable) {
		this.bitfield = this.self.resolve(bits)
	}

	private get self(): Constructor<BitField> & {
		resolve: typeof BitField.resolve
		FLAGS: Record<string, number>
	} {
		return this.constructor as Constructor<BitField> & {
			resolve: typeof BitField.resolve
			FLAGS: Record<string, number>
		}
	}

	missing(bits: BitFieldResolvable): string[] {
		return new this.self(bits).remove(this).toArray()
	}

	any(bit: BitFieldResolvable): boolean {
		return (this.bitfield & this.self.resolve(bit)) !== DEFAULT_BIT
	}

	has(bit: BitFieldResolvable): boolean {
		return (this.bitfield & this.self.resolve(bit)) === bit
	}

	add(...bits: BitFieldResolvable[]): this {
		let total = 0

		for (const bit of bits) {
			total |= this.self.resolve(bit)
		}

		if (Object.isFrozen(this)) return new this.self(this.bitfield | total) as this

		this.bitfield |= total

		return this
	}

	remove(...bits: BitFieldResolvable[]): this {
		let total = 0

		for (const bit of bits) {
			total |= this.self.resolve(bit)
		}

		if (Object.isFrozen(this)) return new this.self(this.bitfield & ~total) as this

		this.bitfield &= ~total

		return this
	}

	freeze(): Readonly<this> {
		return Object.freeze(this)
	}

	valueOf(): number {
		return this.bitfield
	}

	serialize(): Record<string, boolean> {
		const serialized: Record<string, boolean> = {}
		for (const [flag, bit] of Object.entries(this.self.FLAGS)) serialized[flag] = this.has(bit)
		return serialized
	}

	toArray(): string[] {
		const flags = Object.keys(BitField.FLAGS)
		return flags.filter(bit => this.has(bit))
	}

	equals(bit: BitFieldResolvable): boolean {
		return this.bitfield === this.self.resolve(bit)
	}

	*[Symbol.iterator](): Iterable<string> {
		yield* this.toArray()
	}

	static resolve(bit: BitFieldResolvable): number {
		if (typeof bit === 'number') return bit
		if (Array.isArray(bit)) return bit.map(p => this.resolve(p)).reduce((prev, p) => prev | p, DEFAULT_BIT)
		if (bit instanceof BitField) return bit.bitfield
		if (typeof this.FLAGS[bit] !== 'undefined') return this.FLAGS[bit]
		throw new Error('Invalid Bit')
	}
}


BitField.FLAGS = {}