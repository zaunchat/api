import { validator } from './Validator'


const isEmail = validator.compile({
  $$root: true,
  type: 'email'
})

export const email = (str: string) => isEmail(str) === true

export const snowflake = (id: unknown): id is ID => {
  return typeof id === 'string' && /^\d{17,19}$/.test(id)
}

export const nil = (x: unknown): x is null => x == null

export const empty = (obj: unknown): boolean => {
  if (nil(obj)) return true

  if (Array.isArray(obj) && obj.length === 0) return true

  if (typeof obj === 'object') {
    for (const _ in obj) return false
    return true
  }

  return false
}



// https://github.com/fastify/secure-json-parse
const suspiciousProtoRegEx = /"(?:_|\\u005[Ff])(?:_|\\u005[Ff])(?:p|\\u0070)(?:r|\\u0072)(?:o|\\u006[Ff])(?:t|\\u0074)(?:o|\\u006[Ff])(?:_|\\u005[Ff])(?:_|\\u005[Ff])"\s*:/
// https://github.com/hapijs/bourne
const suspiciousConstructorRegEx = /"(?:c|\\u0063)(?:o|\\u006[Ff])(?:n|\\u006[Ee])(?:s|\\u0073)(?:t|\\u0074)(?:r|\\u0072)(?:u|\\u0075)(?:c|\\u0063)(?:t|\\u0074)(?:o|\\u006[Ff])(?:r|\\u0072)"\s*:/

export const suspicious = (value: string): boolean => {
  return suspiciousProtoRegEx.test(value) || suspiciousConstructorRegEx.test(value)
}
