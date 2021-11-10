import isEmail from 'validator/lib/isEmail'

export const email = isEmail



export const snowflake = (id: unknown): id is ID => {
    if (typeof id !== 'string') return false
    return /^\d{17,19}$/.test(id)
}