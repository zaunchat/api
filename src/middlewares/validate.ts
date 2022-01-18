import { CheckError } from "@errors"
import { Request, Response, NextFunction } from "@tinyhttp/app"
import { validator } from "@utils"
import { AsyncCheckFunction, SyncCheckFunction, ValidationSchema } from "fastest-validator"


export const validate = (schema: AsyncCheckFunction | SyncCheckFunction | ValidationSchema, type: 'body' | 'params' | 'query' = 'body'): typeof middleware => {
  const checker = typeof schema === 'function' ? schema : validator.compile(schema)

  const middleware = async (req: Request, _res: Response, next: NextFunction): Promise<void> => {
    const valid = await checker(req[type])

    if (valid !== true) throw new CheckError(valid)

    next()
  }

  return middleware
}
