import { Snowflake } from '../utils'
import sql from '../database'

export abstract class Base {
  id = Snowflake.generate()

  get tableName(): string {
    return `${this.constructor.name.toLowerCase()}s`
  }

  async delete(): Promise<void> {
    await sql`DELETE FROM ${this.tableName} WHERE id = ${this.id}`
  }
}