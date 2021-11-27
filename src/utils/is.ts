import { validator } from './Validator'

const isEmail = validator.compile({
  $$root: true,
  type: 'email'
})

export const email = (str: string) => isEmail(str) === true

export const snowflake = (id: unknown): id is ID => {
  return typeof id === 'string' && /^\d{17,19}$/.test(id)
}


export const empty = (obj: unknown): boolean => {
  if (Array.isArray(obj) && obj.length === 0) return true
  if (obj === null) return true
  if (typeof obj === 'object' && Object.keys(obj).length === 0) return true
  return false
}
