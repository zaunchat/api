import http, { IncomingHttpHeaders, OutgoingHttpHeaders } from 'node:http'
import https from 'node:https'
import { URL } from 'node:url'

interface FetchOptions {
  method: 'GET' | 'POST' | 'DELETE' | 'PUT' | 'PATCH'
  body: Record<string, unknown>
  headers: OutgoingHttpHeaders
  timeout: number
}

interface FetchResponse {
  status: number
  headers: IncomingHttpHeaders
  ok: boolean
  text(): Promise<string>
  json(): Promise<any>
}


export const fetch = (url: string | URL, { method = 'GET', body, headers, timeout = 10000 }: Partial<FetchOptions> = {}): Promise<FetchResponse> => {
  return new Promise(async (ok, err) => {
    if (typeof url === 'string') url = new URL(url)

    const options = {
      hostname: url.hostname,
      port: url.port,
      path: url.pathname + (url.search ?? ''),
      method,
      headers,
      timeout
    }

    const request = (url.protocol === 'https:' ? https : http).request(options, (response) => {
      const data = {
        status: response.statusCode ?? 200,
        headers: response.headers,
        ok: !response.statusCode || (response.statusCode >= 200 && response.statusCode < 300),
        text(): Promise<string> {
          return new Promise(ok => {
            let body = ''
            response
              .setEncoding('utf8')
              .on('data', chunk => body += chunk)
              .on('end', () => ok(body))
          })
        },
        json(): Promise<unknown> {
          return data.text().then(JSON.parse)
        }
      }
      ok(data)
    })

    request
      .on('timeout', () => err(new Error('Request timeout.')))
      .on('error', err)

    if (method !== 'GET' && typeof body !== 'undefined') {
      request.write(JSON.stringify(body))
    }

    request.end()
  })
}
