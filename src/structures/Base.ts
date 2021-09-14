import { PrimaryKey, SerializedPrimaryKey } from '@mikro-orm/core'
import { Snowflake } from '../utils'

export abstract class Base {
  @PrimaryKey({ unique: true })
  _id!: ID

  @SerializedPrimaryKey()
  id!: string

  setID(id?: ID): this {
    this._id = id ?? Snowflake.generate()
    return this
  }
}
