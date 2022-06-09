import fs from 'node:fs/promises'
import { join } from 'node:path'

const CARGO_PATH = join(process.cwd(), 'Cargo.toml')
const content = await fs.readFile(CARGO_PATH, 'utf-8')

await fs.writeFile(CARGO_PATH, content.replace(/version = "(\d+\.\d+\.\d+)"/, `version = "${process.argv[2]}"`))