import {
    User,
    Session,
    Server,
    Channel,
    Member,
    Invite,
    Role,
    Message
} from '../../structures'

const COMMIT = 1;

const run = async () => {
    await User.init()
    await Session.init()
    await Server.init()
    await Channel.init()
    await Member.init()
    await Invite.init()
    await Role.init()
    await Message.init()

    if (COMMIT === 1) {
        // TODO: Unknown
    }
}

export default { run }