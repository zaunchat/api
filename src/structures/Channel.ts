import { Base } from './Base'

export enum ChannelTypes {
    DM,
    TEXT,
    VOICE,
    UNKNOWN
}

export class Channel extends Base {
    type = ChannelTypes.UNKNOWN
}