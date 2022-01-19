import { QueryConfig as QueryBuilder } from 'pg-query-config'
import { Snowflake } from '../utils'
import { HTTPError, APIErrors } from '../errors'
import sql from '../database'

export type WhereFunction<T> = (valueRefSet: Set<T>, args: T[]) => string
export type WhereCondition<T> = { [P in keyof T]?: WhereCondition<T[P]> | T[P] | WhereFunction<T[P]> | Array<T[P]> }
export { QueryConfig as QueryBuilder } from 'pg-query-config'



export abstract class Base {
  readonly id = Snowflake.generate()

  static onCreate?: (self: any) => Awaited<void>
  static onUpdate?: (self: any, keys: string[]) => Awaited<void>
  static onDelete?: (self: any) => Awaited<void>

  get tableName(): string {
    return (this.constructor as typeof Base).tableName
  }

  static get tableName(): string {
    return `${this.name.toLowerCase()}s`
  }

  static async findOne<T>(
    this: (new () => T) & { tableName: string },
    where: WhereCondition<T> | ((query: QueryBuilder<T>) => unknown)
  ): Promise<T> {
    const query = new QueryBuilder<T>({ table: this.tableName, limit: 1 })

    typeof where === 'function' ? where(query) : query.where(where)

    const [item] = await sql.unsafe(query.text, query.values) as [T?]

    if (!item) {
      const tag = `UNKNOWN_${this.name.toUpperCase()}` as keyof typeof APIErrors
      throw new HTTPError(tag)
    }

    return item
  }

  static find<T>(
    this: (new () => T) & { tableName: string },
    where: WhereCondition<T> | ((query: QueryBuilder<T>) => unknown),
    limit = 100
  ): Promise<T[]> {
    const query = new QueryBuilder<T>({ table: this.tableName, limit })

    typeof where === 'function' ? where(query) : query.where(where)

    return sql.unsafe(query.text, query.values) as Promise<T[]>
  }

  static async count(where: string): Promise<number> {
    // TODO: We should return only the count.
    const result = await sql.unsafe(`SELECT * FROM ${this.tableName} WHERE ${where} LIMIT = 1000`)
    return result.count
  }

  async save(): Promise<void> {
    await sql`INSERT INTO ${sql(this.tableName)} ${sql(this)}`;
    (this.constructor as typeof Base).onCreate?.(this)
  }

  async update(props: Partial<NonFunctionProperties<this>>): Promise<this> {
    await sql`UPDATE ${sql(this.tableName)} SET ${sql(props)} WHERE id = ${this.id}`

    Object.assign(this, props);

    (this.constructor as typeof Base).onUpdate?.(this, Object.keys(props))

    return this
  }

  async delete(): Promise<void> {
    await sql`DELETE FROM ${sql(this.tableName)} WHERE id = ${this.id}`;
    (this.constructor as typeof Base).onDelete?.(this)
  }
}
