import { useEffect, useRef, useState } from 'react'
import styles from './Editor.module.scss'

export function Demo({ code, output }: { code: string; output: string }) {
  const outputRef = useRef<HTMLDivElement | null>(null)
  const [typewriter, setTypewriter] = useState<NodeJS.Timeout | null>(null)

  const typewrite = (content: string, speed = 100) => {
    if (content.length && outputRef.current) {
      outputRef.current.innerHTML = outputRef.current.innerHTML + content[0]
      setTimeout(() => typewrite(content.slice(1)), speed)
    }
  }

  useEffect(() => {
    typewrite(code)

    return () => {
      if (outputRef.current) outputRef.current.innerHTML = ''
    }
  }, [])

  return (
    <>
      <pre className="hljs">
        <code ref={outputRef} />
      </pre>
    </>
  )
}

export default function Interpreter() {}
