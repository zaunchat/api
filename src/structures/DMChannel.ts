import { Channel, ChannelTypes } from './Channel'


export class DMChannel extends Channel {
    readonly type = ChannelTypes.DM
}