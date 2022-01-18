import Validator from 'fastest-validator'

export const validator = new Validator({
    useNewCustomCheckerFunction: true,
    defaults: {
        object: {
            strict: true
        },
        email: {
            mode: 'precise'
        }
    }
})

validator.alias('snowflake', {
    type: 'string',
    min: 17,
    max: 19,
    pattern: /^\d{17,19}$/
})