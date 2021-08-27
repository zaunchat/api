export const APIErrors = {
    MISSING_ACCESS: 400,
    MISSING_PERMISSIONS: 400,
    FAILED_CAPTCHA: 403,
    UNAUTHORIZED: 403,
    USERNAME_TAKEN: 409,
    EMAIL_ALREADY_IN_USE: 409,
    USER_NOT_VERIFIED: 403,
    INVALID_PASSWORD: 403,
    EMPTY_MESSAGE: 400,

    // Unknown - 404
    UNKNOWN_ACCOUNT: 404,
    UNKNOWN_USER: 404,
    UNKNOWN_CHANNEL: 404,
    UNKNOWN_MESSAGE: 404,
    UNKNOWN_TOKEN: 404,
    UNKNOWN_SESSION: 404,

    // Maximum
    MAXIMUM_FRIENDS: 400,
    MAXIMUM_SERVERS: 400,
    MAXIMUM_GROUPS: 400,
    MAXIMUM_ROLES: 400,
    MAXIMUM_CHANNELS: 400,
    MAXIMUM_MESSAGE_LENGTH: 400,

    // Misc
    BOT_ONLY: 400,
    USER_ONLY: 400,
    BANNED: 403
} as const

export class HTTPError<T extends keyof typeof APIErrors = keyof typeof APIErrors> {
    message: T
    status: typeof APIErrors[T]
    constructor(key: T) {
        this.message = key
        this.status = APIErrors[key]
    }
}