import { PrimaryKey, Property, Unique } from 'mikro-orm'
import { UUID } from '../utils'

export abstract class Base {
  @PrimaryKey()
  @Unique()
  _id!: string

  setID(): this {
    this._id = UUID.generate()
    return this
  }

  @Property()
  deleted = false
}
