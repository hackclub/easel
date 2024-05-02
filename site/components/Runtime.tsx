import { Nodebox } from '@codesandbox/nodebox'
import { useEffect, useRef } from 'react'

declare global {
  interface Window {
    nodebox: Nodebox
  }
}

export const server = `
import http from "http";
import "./easel.js";

const server = http.createServer((req, res) => {
  res.writeHead(200, {
    "Content-Type": "type/plain"
  })
  res.end("Hello world")
})

server.listen(3000, () => {
})
`

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
import http from "http";

const server = http.createServer((req, res) => {
  res.writeHead(200, {
    "Content-Type": "type/plain"
  })
  res.end("Hello world")
})

server.listen(3000, () => {
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
}

export default function Runtime() {
  const nodeIframe = useRef<HTMLIFrameElement | null>(null)

  useEffect(() => {
    if (nodeIframe.current) loadRuntime(nodeIframe.current)
  }, [])

  return <iframe id="node-iframe" ref={nodeIframe}></iframe>
}
