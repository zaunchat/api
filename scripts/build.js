import { build } from 'esbuild'
import { readdirSync, statSync } from 'node:fs'
import { join, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))

function readdir(...directory) {
  const result = [];

  (function read(dir) {
    const files = readdirSync(dir)
    for (const file of files) {
      const filepath = join(dir, file)
      if (statSync(filepath).isDirectory()) read(filepath)
      else result.push(filepath)
    }
  }(join(...directory)))

  return result
}

await build({
  entryPoints: readdir(__dirname, '../src').filter(file => file.endsWith('.ts')),
  platform: 'node',
  outdir: 'dist',
  minify: true,
  target: 'node16'
})


console.log('Compiled')
