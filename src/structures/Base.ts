import { PrimaryKey, Property } from 'mikro-orm'
import { UUID } from '../utils'

export abstract class Base {
  @PrimaryKey({ unique: true })
  _id!: string

  setID(): this {
    this._id = UUID.generate()
    return this
  }

  @Property()
  deleted = false
}
