import { validator } from './validator'

const isEmail = validator.compile({
    $$root: true,
    type: "email"
})

export const email = (str: string) => isEmail(str) === true
export const snowflake = (id: unknown): id is ID => typeof id === 'string' && /^\d{17,19}$/.test(id)