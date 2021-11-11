import isEmail from 'validator/lib/isEmail'

export const email = isEmail

export const snowflake = (id: unknown): id is ID => typeof id === 'string' && /^\d{17,19}$/.test(id)