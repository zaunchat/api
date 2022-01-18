import { build } from 'esbuild'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { readdirSync, statSync } from 'node:fs'

const __dirname = dirname(fileURLToPath(import.meta.url)), start = Date.now()


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
  entryPoints: readdir(__dirname, '../src'),
  format: 'esm',
  platform: 'node',
  outdir: 'dist',
  minify: false,
  target: 'node16'
})

const now = Date.now()

console.log(`⚡ Done in ${now - start}ms`)
