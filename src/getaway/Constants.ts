import { WSEvents } from "../@types"

export interface Payload {
    code: WSCodes
    event?: keyof WSEvents
    data?: unknown
}

export enum WSCodes {
    HELLO,
    PING,
    PONG,
    AUTHENTICATE,
    AUTHENTICATED,
    READY
}


export enum WSCloseCodes {
    UNKNOWN_ERROR = 4000,
    UNKNOWN_OPCODE,
    DECODE_ERROR,
    NOT_AUTHENTICATED,
    AUTHENTICATED_FAILED,
    ALREADY_AUTHENTICATED,
    INVALID_SESSION,
    RATE_LIMITED,
    SESSION_TIMEOUT
}