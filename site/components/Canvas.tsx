import { useEffect, useRef, useState } from 'react'
import stdlib, { Canvas as ParentCanvas } from '../../languages/easel/stdlib'
import { Lexer } from '../../languages/easel/lexer'
import { Parser } from '../../languages/easel/parser'
import { Interpreter } from '../../languages/easel/interpreter'
import styles from './Editor.module.scss'
import ReactCodeMirror from '@uiw/react-codemirror'
import { quietlight } from '@uiw/codemirror-theme-quietlight'

export function Easel({
  code,
  lib,
  defaultColor = '#ddd',
  gap = 2,
  width = 64,
  height = 64,
  run
}: {
  code: string
  lib: { [key: string]: (args: any[]) => any }
  defaultColor?: string
  gap?: number
  width?: number
  height?: number
  run: boolean
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  const clear = (
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number,
    cellSize: number
  ) => {
    // Restore grid
    ctx.fillStyle = 'white'
    ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)
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
    if (run) {
      if (canvasRef.current) {
        const canvas = canvasRef.current
        const ctx = canvas.getContext('2d')

        if (ctx && canvas.parentElement) {
          canvas.width = canvas.parentElement.offsetWidth
          canvas.height = canvas.width

          let cellSize = canvas.width / width - gap

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
                ctx.fillStyle = defaultColor
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
          const interpreter = new Interpreter()
          let scope = interpreter.run(parser.ast, {
            ...stdlib,
            ...lib,
            Canvas: new CustomCanvas()
          })

          const interval: NodeJS.Timeout = setInterval(() => {
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

          window.addEventListener('resize', () => {
            if (canvas.parentElement) {
              canvas.width = canvas.parentElement.offsetWidth
              canvas.height = canvas.width
              cellSize = ctx.canvas.width / width - gap
              clear(ctx, width, height, cellSize)
            }
          })

          return () => {
            clearInterval(interval)
            clear(ctx, width, height, cellSize)
          }
        }
      }
    } else if (canvasRef.current) {
      // When paused, clear canvas
      const canvas = canvasRef.current
      const ctx = canvas.getContext('2d')

      if (ctx && canvas.parentElement) {
        canvas.width = canvas.parentElement.offsetWidth
        canvas.height = canvas.width
        clear(ctx, width, height, ctx.canvas.width / width - gap)
      }
    }
  }, [run])

  return <canvas className={styles.easel} ref={canvasRef} />
}

export default function Canvas({
  initialCode = '',
  initialRun = true,
  editable = true
}: {
  initialCode: string
  initialRun: boolean
  editable: boolean
}) {
  const gridRef = useRef<HTMLDivElement>(null)
  const wrapperRef = useRef<HTMLDivElement>(null)
  const [code, setCode] = useState(initialCode)
  const [output, setOutput] = useState<string[]>([])
  const [height, setHeight] = useState('1px')
  const [run, setRun] = useState<boolean>(initialRun)

  useEffect(() => {
    // if (gridRef.current) {
    //   const grid = gridRef.current
    //   if (grid.parentElement) {
    //     setHeight(`${grid.parentElement.parentElement.offsetHeight}px`)
    //   }
    // }

    if (wrapperRef.current) {
      const wrapper = wrapperRef.current
      setHeight(`${wrapper.offsetHeight + 100}px`)
    }

    return () => setOutput([])
  }, [])

  return (
    <div className={styles.editor} ref={wrapperRef}>
      <div className={styles.editable}>
        <ReactCodeMirror
          height={height}
          theme={quietlight}
          onChange={value => {
            setCode(value)
          }}
          value={code}
          editable={editable}
        />
      </div>
      <div>
        <div className={styles.tabs} style={{ borderTopWidth: '0 !important' }}>
          <div
            className={styles.tab}
            onClick={() => {
              setOutput([])
              setRun(false)
            }}>
            Clear
          </div>
          <div
            className={styles.tab}
            onClick={() => {
              if (!run) {
                setOutput([])
                setRun(true)
              }
            }}>
            Run
          </div>
        </div>
        <div ref={gridRef}>
          <Easel
            code={code}
            lib={{
              ink: (args: string[]): void => {
                setOutput(old => [
                  ...old,
                  ...args.map(arg => JSON.stringify(arg))
                ])
              }
            }}
            run={run}
          />
          <div className={styles.terminal}>
            {output.length ? (
              output.map((line, idx) => <code key={idx}>{line}</code>)
            ) : (
              <code>
                <i>Output will show up here.</i>
              </code>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}
