import fs from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'


const __dirname = path.dirname(fileURLToPath(import.meta.url))


async function fetchControllers(dir) {
  const result = [];
  const filter = file => ['index.ts', '/Controller.ts', 'decorators.ts'].every(name => !file.endsWith(name))

  async function read(dir) {
    const files = await fs.readdir(dir)

    for (const filepath of files.map((p) => path.join(dir, p))) {
      const stat = await fs.stat(filepath)

      if (stat.isDirectory()) {
        await read(filepath)
      } else {
        result.push(filepath)
      }
    }
  }

  await read(dir)

  for (let i = 0; i < result.length; i++) {
    const filePath = result[i]

    if (!filter(filePath)) {
      delete result[i]
      continue
    }

    result[i] = {
      path: filePath,
      content: await fs.readFile(filePath, 'utf8')
    }
  }

  return result.filter(Boolean)
}

async function parse(controller) {

  function getPath() {
    return controller.content.match(/extends Controller\('(.+)'\)/)[1]
  }


  function getRoutes() {
    const routes = []
    const regex = /'(GET|POST|DELETE|PATCH|PUT) (.+)'\(_?ctx: Context\)(?:: Promise<(.+)> {)?/g

    let result;

    while (result = regex.exec(controller.content)) {
      const [, method, path, type = 'unknown'] = result
      routes.push({ method, path, type })
    }

    return routes
  }


  controller.path = 'TODO: Change path to docs dir.'
  controller.content = {
    basePath: getPath(),
    routes: getRoutes()
  }

  console.log(controller)


  return controller
}

async function save(controller) {
  console.log(controller)
  //await fs.writeFile(controller.path, JSON.stringify(controller.content, null, 2))
}


fetchControllers(path.join(__dirname, '../src/controllers')).then(controllers => {
  return Promise.all(controllers.map(parse))
}).then(controllers => {
  return Promise.all(controllers.map(save))
}).then(({ length: size }) => {
  console.log('Done!')
  console.log(`Size: ${size}`)
}).catch(err => {
  console.error(`Failed generate docs for the following reason: `, err)
  process.exit(-1)
})
