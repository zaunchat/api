export enum APIErrors {
    MISSING_ACCESS,
    MISSING_PERMISSIONS,
    FIELD_CAPTCHA,
    UNAUTHORIZED,

    // Unknown
    UNKNOWN_ACCOUNT,
    UNKNOWN_USER,
    UNKNOWN_CHANNEL,
    UNKNOWN_MESSAGE,
    UNKNOWN_TOKEN,
    UNKNOWN_SESSION,

    // Maximum
    MAXIMUM_FRIENDS,
    MAXIMUM_SERVERS,
    MAXIMUM_GROUPS,
    MAXIMUM_ROLES,
    MAXIMUM_CHANNELS,
    MAXIMUM_MESSAGE_LENGTH,


    // Misc
    BOT_ONLY,
    USER_ONLY
}

export class HTTPError<T extends keyof typeof APIErrors = keyof typeof APIErrors> {
    message: T
    code: typeof APIErrors[T]
    constructor(key: T) {
        this.message = key
        this.code = APIErrors[key]
    }
}