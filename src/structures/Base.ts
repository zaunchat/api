import { QueryConfig as QueryBuilder } from 'pg-query-config'
import { Snowflake } from '../utils'
import { HTTPError, APIErrors } from '../errors'
import sql from '../database'

export type WhereFunction<T> = (valueRefSet: Set<T>, args: T[]) => string
export type WhereCondition<T> = { [P in keyof T]?: WhereCondition<T[P]> | T[P] | WhereFunction<T[P]> | Array<T[P]> }
export { QueryConfig as QueryBuilder } from 'pg-query-config'



export abstract class Base {
  readonly id = Snowflake.generate()

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
    const [{ count }] = await sql.unsafe(`SELECT COUNT(id) as count FROM ${this.tableName} WHERE ${where}`)
    return count
  }

  async save(): Promise<void> {
    await sql`INSERT INTO ${sql(this.tableName)} ${sql(this as any)}`
  }

  async update(props: Partial<NonFunctionProperties<this>>): Promise<this> {
    await sql`UPDATE ${sql(this.tableName)} SET ${sql(props as any)} WHERE id = ${this.id}`

    Object.assign(this, props)

    return this
  }

 async delete(): Promise<void> {
    await sql`DELETE FROM ${sql(this.tableName)} WHERE id = ${this.id}`
  }
}
