import { useState } from 'react'
import styles from './Editor.module.scss'

export default function Interpreter() {
  const [output, setOutput] = useState<Array<{ type: string; value: string }>>(
    []
  )

  return (
    <div className="interactive">
      <div className={styles.terminal}>
        {output.map((line, idx) => (
          <code key={idx}>{line.value}</code>
        ))}
        <code style={{ display: 'flex' }}>
          &gt; <span contentEditable style={{ minWidth: '100%' }}></span>
        </code>
      </div>
    </div>
  )
}
