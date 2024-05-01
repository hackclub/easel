import { useEffect, useRef, useState } from 'react'
import styles from './Editor.module.scss'
import CodeMirror from '@uiw/react-codemirror'
import { quietlight } from '@uiw/codemirror-theme-quietlight'
import stdlib, { Canvas as ParentCanvas } from '../../easel/stdlib'
import { Lexer } from '../../easel/lexer'
import { Parser } from '../../easel/parser'
import { Interpreter } from '../../easel/interpreter'

export function Easel({
  code,
  defaultColor = '#ddd',
  gap = 2
}: {
  code: string
  defaultColor?: string
  gap?: number
}) {
  const canvasRef = useRef<HTMLCanvasElement | null>(null)

  const clear = (
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number,
    cellSize: number
  ) => {
    // Restore grid
    ctx.fillStyle = defaultColor
    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        ctx.fillRect(
          x * cellSize + gap * x,
          y * cellSize + gap * y,
          cellSize,
          cellSize
        )
      }
    }
  }

  useEffect(() => {
    if (canvasRef.current) {
      const canvas = canvasRef.current
      const ctx = canvas.getContext('2d')

      if (ctx && canvas.parentElement) {
        canvas.width = canvas.parentElement.offsetWidth
        canvas.height = canvas.width

        const width = 64
        const height = 64
        const cellSize = ctx.canvas.width / width - gap

        clear(ctx, width, height, cellSize)

        class CustomCanvas extends ParentCanvas {
          fill([x, y, color]: [
            number,
            number,
            { r: number; g: number; b: number }
          ]) {
            let cell = this.grid[y * this.cols + x]
            if (cell) {
              cell.r = color.r
              cell.g = color.g
              cell.b = color.b

              // Set actual cell color to that
              if (ctx) {
                ctx.fillStyle = `rgb(${cell.r}, ${cell.g}, ${cell.b})`
                ctx.fillRect(
                  x * cellSize + gap * x,
                  y * cellSize + gap * y,
                  cellSize,
                  cellSize
                )
              }
            }
          }

          erase([x, y]: [number, number]) {
            if (ctx) {
              ctx.fillStyle = '#ddd'
              ctx.fillRect(
                x * cellSize + gap * x,
                y * cellSize + gap * y,
                cellSize,
                cellSize
              )
            }
          }
        }

        const lexer = new Lexer(code)
        lexer.scanTokens()
        const parser = new Parser(lexer.tokens)
        parser.parse()
        console.log(parser.ast)
        const interpreter = new Interpreter()
        let scope = interpreter.run(parser.ast, {
          ...stdlib,
          Canvas: new CustomCanvas()
        })

        const interval: any = setInterval(() => {
          if (!interpreter.inScope(scope, 'painting'))
            return clearInterval(interval)
          ctx.fillStyle = 'white'
          ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)
          try {
            const lexer = new Lexer('painting()')
            lexer.scanTokens()
            const parser = new Parser(lexer.tokens)
            parser.parse()
            scope = interpreter.run(parser.ast, scope)
          } catch {
            clearInterval(interval)
          }
        }, 100)

        return () => {
          ctx.fillStyle = 'white'
          ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)
          clearInterval(interval)
        }
      }
    }
  }, [code])

  return <canvas className={styles.easel} ref={canvasRef} />
}

export default function Canvas() {
  const gridRef = useRef<HTMLDivElement | null>(null)
  const [code, setCode] = useState('')
  const [current, setCurrent] = useState('')
  const [height, setHeight] = useState('1px')

  useEffect(() => {
    if (gridRef.current) {
      const grid = gridRef.current
      setHeight(`${grid.parentElement?.offsetHeight}px`)
    }
  }, [])

  return (
    <div className={styles.editor} style={{ borderRight: 'none !important' }}>
      <div className={styles.editable}>
        <CodeMirror
          height={height}
          theme={quietlight}
          onChange={value => {
            setCurrent(value)
          }}
        />
      </div>
      <div>
        <div className={styles.tabs} style={{ justifyContent: 'flex-end' }}>
          <div
            className={styles.tab}
            style={{ borderLeft: '1px solid var(--border)' }}
            onClick={() => setCode(current)}>
            Run
          </div>
        </div>
        <div ref={gridRef}>
          <Easel code={code} />
        </div>
      </div>
    </div>
  )
}
