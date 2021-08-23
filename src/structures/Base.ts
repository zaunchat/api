import { PrimaryKey, Property } from 'mikro-orm'
import { UUID } from '../utils/UUID'

export abstract class Base {
  @PrimaryKey({ onCreate: () => UUID.generate() })
  _id!: string
  
  @Property()
  deleted = false
}
