import { Nodebox, ShellProcess } from "@codesandbox/nodebox"
import { useEffect, useRef } from "react"

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
    "package.json": JSON.stringify({
      name: "easel"
    }),
    "index.js": "",
    "ast.js": "",
    ""
  })
}