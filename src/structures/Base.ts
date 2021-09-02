import { PrimaryKey, Property } from 'mikro-orm'
import { SnowflakeUtil, Snowflake } from '../utils'

export abstract class Base {
  @PrimaryKey({ unique: true })
  _id!: Snowflake

  setID(): this {
    this._id = SnowflakeUtil.generate()
    return this
  }

  @Property()
  deleted = false
}
