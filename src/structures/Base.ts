// deno-lint-ignore-file no-explicit-any
import { Snowflake, logger } from '../utils'
import sql from '../database'
import { QueryConfig as QueryBuilder } from 'pg-query-config';
import { HTTPError, APIErrors } from '../errors';

type WhereFunction<T> = (valueRefSet: Set<T>, args: T[]) => string
type WhereCondition<T> = {
  [P in keyof T]?: WhereCondition<T[P]> | T[P] | WhereFunction<T[P]> | Array<T[P]>
}


export abstract class Base {
  readonly id = Snowflake.generate()

  static async onUpdate(_self: unknown): Promise<void> {
    logger.warn(`Unhandled method at ${this.tableName}`)
  }

  static async onCreate(_self: unknown): Promise<void> {
    logger.warn(`Unhandled method at ${this.tableName}`)
  }

  static async onDelete(_self: unknown): Promise<void> {
    logger.warn(`Unhandled method at ${this.tableName}`)
  }

  get tableName(): string {
    return `${this.constructor.name.toLowerCase()}s`
  }

  static get tableName(): string {
    return `${this.name.toLowerCase()}s`
  }

  static async findOne<T>(
    this: new () => T,
    where: WhereCondition<T> | ((query: QueryBuilder<T>) => QueryBuilder<T>)
  ): Promise<T> {
    let query = new QueryBuilder<T>({ table: (<any>this).tableName, limit: 1 })

    if (typeof where === 'function') {
      query = where(query)
    } else {
      query.where(where)
    }

    const [item] = await sql.unsafe(query.text, query.values) as [T?]

    if (!item) {
      const tag = `UNKNOWN_${this.name.toUpperCase()}` as keyof typeof APIErrors

      if (!(tag in APIErrors)) {
        throw new Error('Unhandled type')
      }

      throw new HTTPError(tag)
    }

    return item
  }

  static find<T>(
    this: new () => T,
    where: WhereCondition<T> | ((query: QueryBuilder<T>) => QueryBuilder<T>),
    limit = 100
  ): Promise<T[]> {
    let query = new QueryBuilder<T>({ table: (<any>this).tableName, limit })

    if (typeof where === 'function') {
      query = where(query)
    } else {
      query.where(where)
    }

    return sql.unsafe(query.text, query.values) as Promise<T[]>
  }

  static async count(where: string): Promise<number> {
    const result = await sql.unsafe(`SELECT * FROM ${this.tableName} WHERE ${where} LIMIT = 1000`)
    return result.count
  }

  async save(): Promise<void> {
    // TODO: Better handling
    const clone = { ...this } as unknown as Record<string, string>

    // Issue: https://github.com/porsager/postgres/issues/242
    for (const [key, value] of Object.entries(clone)) {
      if (value != null && typeof value === 'object') clone[key] = JSON.stringify(value)
    }

    await sql`INSERT INTO ${sql(this.tableName)} ${sql(clone)}`

    void (this.constructor as any).onCreate(this)
  }

  async update(props: Partial<this>): Promise<this> {
    const [data] = await sql<unknown[]>`UPDATE ${this.tableName} SET ${sql(props)} WHERE id = ${this.id} RETURNING *`

    void (this.constructor as any).onUpdate(this)

    return Object.assign(this, data)
  }

  async delete(): Promise<void> {
    await sql`DELETE FROM ${sql(this.tableName)} WHERE id = ${this.id}`
    void (this.constructor as any).onDelete(this)
  }
}
