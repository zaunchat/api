import postgres from 'postgres'
import config from '../config'

const noop = () => { }

export default postgres(config.database.uri, {
  debug: console.log,
  onnotice: noop,
  types: {
    number: {
      to: 0,
      from: [21, 23, 26, 700, 701],
      serialize: x => {
        // if (typeof x === 'object') return JSON.stringify(x)
        return '' + x
      },
      parse: x => +x
    }
  }
})
