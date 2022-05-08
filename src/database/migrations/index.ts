import sql from '..';

const DATE = '2022-4'

const run = async (): Promise<void> => {
  await sql.file(`./${DATE}.sql`)
}

export default { run }
