import mermaid from 'mermaid'
import { useEffect } from 'react'

mermaid.initialize({
  startOnLoad: true,
  theme: 'default',
  securityLevel: 'none'
})

type MermaidProps = {
  readonly chart: string
}

export default function Mermaid({ chart }: MermaidProps): JSX.Element {
  useEffect(() => {
    mermaid.contentLoaded()
  }, [])

  return (
    <div className="interactive">
      <div className="mermaid">{chart}</div>
    </div>
  )
}
