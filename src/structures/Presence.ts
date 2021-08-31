export interface CreatePresenceOptions extends Partial<Presence> {}

export enum PresenceStatus {
	ONLINE,
	OFFLINE,
	IDLE,
	DND
}

export class Presence {
	status = PresenceStatus.OFFLINE
	static from(options: CreatePresenceOptions): Presence {
		const presence = new Presence()
		Object.assign(presence, options)
		return presence
	}
}