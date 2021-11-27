export const APIErrors = {
  MISSING_ACCESS: [400, 'Missing access'],
  MISSING_PERMISSIONS: [400, 'You lack permissions to perform that action'],
  FAILED_CAPTCHA: [403],
  UNAUTHORIZED: [403, 'Unauthorized. Provide a valid token and try again'],
  ACCOUNT_VERIFICATION_REQUIRED: [400, 'You need to verify your account in order to perform this action'],
  USERNAME_TAKEN: [409],
  EMAIL_ALREADY_IN_USE: [409],
  USER_NOT_VERIFIED: [403],
  INVALID_PASSWORD: [403, 'Invalid password'],
  INVALID_EMAIL: [403, 'Invalid email'],
  EMPTY_MESSAGE: [400, 'Cannot send an empty message'],
  TOO_MANY_ATTACHMENTS: [400],
  TOO_MANY_REPLIES: [400],
  INVALID_ID: [400, 'Invalid ID'],
  BLOCKED: [409],
  BLOCKED_BY_OTHER: [403],
  ALREADY_SENT_REQUEST: [409],
  ALREADY_FRIENDS: [409],

  // Unknown - 404
  UNKNOWN_ACCOUNT: [404, 'Unknown account'],
  UNKNOWN_USER: [404, 'Unknown user'],
  UNKNOWN_CHANNEL: [404, 'Unknown channel'],
  UNKNOWN_MESSAGE: [404, 'Unknown message'],
  UNKNOWN_TOKEN: [404, 'Unknown token'],
  UNKNOWN_SESSION: [404, 'Unknown session'],
  UNKNOWN_SERVER: [404, 'Unknown server'],
  UNKNOWN_MEMBER: [404, 'Unknown member'],
  UNKNOWN_ROLE: [404, 'Unknown role'],
  UNKNOWN_GROUP: [404, 'Unknown group'],
  UNKNOWN_INVITE: [404, 'Unknown invite'],

  // Maximum
  MAXIMUM_FRIENDS: [400, 'Maximum number of friends reached'],
  MAXIMUM_SERVERS: [400, 'Maximum number of servers reached'],
  MAXIMUM_GROUPS: [400, 'Maximum number of groups reached'],
  MAXIMUM_ROLES: [400, 'Maximum number of roles reached'],
  MAXIMUM_CHANNELS: [400, 'Maximum number of channels reached'],
  MAXIMUM_MESSAGE_LENGTH: [400],
  MAXIMUM_GROUP_MEMBERS: [400, 'Maximum number of group members reached'],

  // Misc
  BOT_ONLY: [400],
  USER_ONLY: [400],
  BANNED: [403, 'The user is banned from this server'],
  CANNOT_EDIT_MESSAGE_BY_OTHER: [400, 'Cannot edit a message authored by another user']
} as const


export class HTTPError<T extends keyof typeof APIErrors> {
  message: string
  status: number
  constructor(key: T) {
    const [status, message] = APIErrors[key] ?? [404, 'Unknown error']
    this.message = message ?? key
    this.status = status
  }
}
