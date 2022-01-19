import { User } from '../structures'
import { Permissions } from '../utils'

declare module '@tinyhttp/app' {
  interface Request {
    user: User
    private permissions?: Permissions
  }
}

declare global {
  type ID = string
  type Awaited<T> = T | Promise<T>
  type WithFlag<T, Flag> = T | `${T} --${Flag}`
  type Nullable<T> = T | null
  type FunctionPropertyNames<T> = { [K in keyof T]: T[K] extends Function ? K : never }[keyof T]
  type NonFunctionProperties<T> = Omit<T, FunctionPropertyNames<T>>
  type Options<T> = Partial<NonFunctionProperties<T>>
}

declare module 'postgres' {
  interface Options<T extends JSToPostgresTypeMap> extends Partial<BaseOptions<T>> {
    publications?: string
  }

  interface Sql<TTypes extends JSToPostgresTypeMap> {
    subscribe(pattern: string, fn: (...args: any[]) => Awaited<unknown | void>): Promise<{ unsubscribe: () => Promise<void> }>
  }
}
