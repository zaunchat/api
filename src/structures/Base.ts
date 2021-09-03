import { PrimaryKey } from 'mikro-orm'
import { SnowflakeUtil } from '../utils'

export abstract class Base {
  @PrimaryKey({ unique: true })
  _id!: Snowflake

  setID(): this {
    this._id = SnowflakeUtil.generate()
    return this
  }
}
