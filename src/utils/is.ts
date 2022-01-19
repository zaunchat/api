import { Payload, WSCodes } from '../getaway/Constants'
import { validator } from './Validator'

const isEmail = validator.compile({
  $$root: true,
  type: 'email'
})

const isPayload = validator.compile({
  code: {
    type: 'enum',
    values: Object.keys(WSCodes).filter(k => !isNaN(+k))
  },
  data: {
    type: 'any',
    optional: true
  }
})

export const email = (str: string) => isEmail(str) === true

export const payload = (obj: unknown): obj is Payload => isPayload(obj) === true

export const snowflake = (id: unknown): id is ID => {
  if (typeof id !== 'string') return false

  if (id.length < 17 || id.length > 19) return false

  return /^\d{17,19}$/.test(id)
}

export const nil = (x: unknown): x is null | undefined => x == null


export const object = (x: unknown): x is Record<string, unknown> => {
  return typeof x === 'object' && !nil(x) && !Array.isArray(x)
}

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
  // Quick test
  if (value === '' || value === 'null' || value === '{}') return false
  return suspiciousProtoRegEx.test(value) || suspiciousConstructorRegEx.test(value)
}
