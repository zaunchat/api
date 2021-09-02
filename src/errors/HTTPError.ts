export const APIErrors = {
    MISSING_ACCESS: [400, 'Missing access'],
    MISSING_PERMISSIONS: [400, 'You lack permissions to perform that action'],
    FAILED_CAPTCHA: [403],
    UNAUTHORIZED: [403, 'Unauthorized. Provide a valid token and try again'],
    ACCOUNT_VERIFICATION_REQUIRED: [400, 'You need to verify your account in order to perform this action'],
    USERNAME_TAKEN: [409],
    EMAIL_ALREADY_IN_USE: [409],
    USER_NOT_VERIFIED: [403],
    INVALID_PASSWORD: [403],
    EMPTY_MESSAGE: [400, 'Cannot send an empty message'],
    TOO_MANY_ATTACHMENTS: [400],
    TOO_MANY_REPLIES: [400],

    // Unknown - 404
    UNKNOWN_ACCOUNT: [404, 'Unknown account'],
    UNKNOWN_USER: [404, 'Unknown user'],
    UNKNOWN_CHANNEL: [404, 'Unknown channel'],
    UNKNOWN_MESSAGE: [404, 'Unknown message'],
    UNKNOWN_TOKEN: [404, 'Unknown token'],
    UNKNOWN_SESSION: [404, 'Unknown session'],
    UNKNOWN_SERVER: [404, 'Unknwon server'],
    UNKNOWN_MEMBER: [404, 'Unknown member'],
    UNKNOWN_ROLE: [404, 'Unknown role'],

    // Maximum
    MAXIMUM_FRIENDS: [400, 'Maximum number of freinds reached'],
    MAXIMUM_SERVERS: [400, 'Maximum number of servers reached'],
    MAXIMUM_GROUPS: [400, 'Maximum number of groups reached'],
    MAXIMUM_ROLES: [400, 'Maximum number of roles reached'],
    MAXIMUM_CHANNELS: [400, 'Maximum number of channels reached'],
    MAXIMUM_MESSAGE_LENGTH: [400],

    // Misc
    BOT_ONLY: [400],
    USER_ONLY: [400],
    BANNED: [403, 'The user is banned from this server'],
    CANNOT_EDIT_MESSAGE_BY_OTHER: [400, 'Cannot edit a message authored by another user']
} as const

export class HTTPError<T extends keyof typeof APIErrors = keyof typeof APIErrors> {
    message: T | typeof APIErrors[T][1]
    status: typeof APIErrors[T][0]
    constructor(key: T) {
        const [status, message] = APIErrors[key]
        this.message = message ?? key
        this.status = status
    }
}


export class CheckError {
    readonly status = 400
    constructor(public message: unknown) {}
}