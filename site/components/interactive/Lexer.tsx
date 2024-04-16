import { Lexer, Token, TOKENS } from '../../../easel/lexer'
import { useEffect, useState } from 'react'
import styles from './Lexer.module.scss'

export function TokenComponent({
  type,
  value,
  content
}: {
  type: string
  value: string
  content: string
}) {
  const [popover, setPopover] = useState(false)

  return (
    <code
      className={styles.token}
      onMouseEnter={() => setPopover(true)}
      onMouseLeave={() => setPopover(false)}>
      <code>{content}</code>
      {popover && (
        <div className={styles.popover}>
          <p>
            <code>{content}</code> is a {type}
          </p>
          <p>{JSON.stringify({ type, value, content })}</p>
        </div>
      )}
    </code>
  )
}

export default function LexerComponent() {
  const [code, setCode] = useState(`sketch painting {
  ~ This loop runs every iteration and must be in every program
  loop i through (0, cells.length()) {
    prepare cell as cells[i]
    prepare neighbors as paint getNeighbors with (cells, i)
    if (cell.live) {
      if (neigbors.length() < 2 || neighbors.length() > 3) {
        ~ Any live cell with fewer than two neighbors dies, as if by underpopulation
        ~ Any live cell with more than three live neighbors dies, as if by overpopulation
        cell.live = false
      }
    } elif (!(cell.live && neighbors == 3)) {
      ~ Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction
      cell.live = true
    }

    if (cell.live) {
      ~ Now draw the cell if it's alive!
      prepare color as Color(r: 0, g: 255, b: 0)
      Canvas.fill(x, y, color)
    } else {
      ~ If it's dead, turn the cell off
      Canvas.erase(x, y)
    }
  }
}`)
  const [tokens, setTokens] = useState<Token[]>([])

  useEffect(() => {
    const lexer = new Lexer(code)
    lexer.scanTokens()
    setTokens(lexer.tokens)
  }, [])

  return (
    <div className="interactive">
      <div className={styles.wrapper}>
        <pre className={styles.tokens}>
          {tokens.map((token, idx) => {
            // Restore missing whitespace
            const newline = code.split('\n')[token.line]
            if (newline) {
              const prev = newline[token.column]
              if (prev === ' ')
                return (
                  <>
                    <code> </code>
                    <TokenComponent key={idx} {...token} />
                  </>
                )
            }
            if (newline && newline.trim().startsWith(token.value)) {
              return (
                <>
                  <div style={{ width: '100%' }} />
                  <code>{'\n  '}</code>
                  <TokenComponent key={idx} {...token} />
                </>
              )
            }
            return <TokenComponent key={idx} {...token} />
          })}
        </pre>
      </div>
    </div>
  )
}
