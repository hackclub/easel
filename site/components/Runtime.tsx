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

const server = http.createServer(async (req, res) => {
  req.url = req.url.replace("/", "")
  console.log(req.url)
  if (req.url === "program.easel") {
    try {
      const content = fs.readFileSync("program.easel", "utf-8")
      res.writeHead(200, {
        "Content-Type": "text/plain"
      })
      res.write(content)
      res.end()
    } catch (err) {
      console.log(err)
    }
  } else if (req.url === "lexer.js") {
    const content = fs.readFileSync("lexer.js", "utf-8")
    res.writeHead(200, {
      "Content-Type": "text/javascript"
    })
    res.write(content)
    res.end()
  } else if (req.url === "stdlib.js") {
    const content = fs.readFileSync("stdlib.js", "utf-8")
    res.writeHead(200, {
      "Content-Type": "text/javascript"
    })
    res.write(content)
    res.end()
  } else {
    res.writeHead(200, {
      "Content-Type": "text/html"
    })
    res.write(fs.readFileSync("index.html", "utf-8"))
    res.end()
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
