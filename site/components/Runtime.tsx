import { Nodebox, ShellProcess } from '@codesandbox/nodebox'
import { useEffect, useRef } from 'react'

declare global {
  interface Window {
    nodebox: Nodebox
    shells: ShellProcess[]
  }
}

export async function loadRuntime(iframe: HTMLIFrameElement) {
  const runtime = new Nodebox({
    iframe
  })

  await runtime.connect()

  await runtime.fs.init({
    'package.json': JSON.stringify({
      name: 'easel'
      // type: 'module'
    }),
    'index.js': `
import http from "http"
import fs from "fs"

const returnFile = (res, mime, file) => {
  const content = fs.readFileSync(file, "utf-8")
  res.writeHead(200, {
    "Content-Type": mime
  })
  res.write(content)
  res.end()
}

const server = http.createServer(async (req, res) => {
  req.url = req.url.replace("/", "")
  switch (req.url) {
    case "program.easel":
      return returnFile(res, "text/plain", "program.easel")
    case "test.easel":
      return returnFile(res, "text/plain", "test.easel")
    case "lexer.js":
      return returnFile(res, "text/javascript", "lexer.js")
    case "parser.js":
      return returnFile(res, "text/javascript", "parser.js")
    case "interpreter.js":
      return returnFile(res, "text/javascript", "interpreter.js")
    case "ast.js":
      return returnFile(res, "text/javascript", "ast.js")
    case "stdlib.js":
      return returnFile(res, "text/javascript", "stdlib.js")
    default:
      return returnFile(res, "text/html", "index.html")
  }
})

server.listen(3000, async () => {
  require("./easel.js")(process.argv)
})
`,
    'ast.js': '',
    'easel.js': '',
    'interpreter.js': '',
    'lexer.js': '',
    'parser.js': '',
    'stdlib.js': '',
    'program.easel': '',
    'test.easel': ''
  })

  window.nodebox = runtime
  window.shells = []
}

export default function Runtime() {
  const nodeIframe = useRef<HTMLIFrameElement | null>(null)

  useEffect(() => {
    if (nodeIframe.current) loadRuntime(nodeIframe.current)
  }, [])

  return <iframe id="node-iframe" ref={nodeIframe}></iframe>
}
