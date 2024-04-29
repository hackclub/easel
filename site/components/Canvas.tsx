import { useEffect, useRef, useState } from 'react'
import CodeMirror from '@uiw/react-codemirror'
import styles from './Editor.module.scss'
import { quietlight } from '@uiw/codemirror-theme-quietlight'
import { FaCirclePlay } from 'react-icons/fa6'
import { Lexer } from '../../easel/lexer'
import { Parser } from '../../easel/parser'
import { Interpreter } from '../../easel/interpreter'
import stdlib, { Canvas as ParentCanvas } from '../../easel/stdlib'

class Cell {
  x: number
  y: number
  size: number
  color: string

  constructor(x: number, y: number, size: number, color = '#ddd') {
    this.x = x
    this.y = y
    this.size = size
    this.color = color
  }

  get element() {
    let td = document.createElement('div')
    td.style.width = `${this.size}px`
    td.style.height = `${this.size}px`
    td.style.backgroundColor = this.color
    return td
  }
}

export function Easel() {
  const gridRef = useRef<HTMLDivElement | null>(null)

  useEffect(() => {
    if (gridRef.current) {
      const grid = gridRef.current
      const gridWidth = grid.parentElement?.offsetWidth || 500
      grid.style.height = `${gridWidth}px`

      const init = (width: number, height: number, gap = 2) => {
        let cellSize = gridWidth / width - gap
        for (let y = 0; y < height; y++) {
          let tr = document.createElement('div')
          tr.style.height = `${cellSize}px`
          for (let x = 0; x < width; x++) {
            let td = new Cell(x, y, cellSize)
            tr.appendChild(td.element)
          }
          grid.appendChild(tr)
        }
      }

      init(64, 64)
    }

    return () => {
      if (gridRef.current) gridRef.current.innerHTML = ''
    }
  }, [])

  return (
    <div className={styles.easel}>
      <div ref={gridRef} />
    </div>
  )
}

export default function Canvas() {
  const [code, setCode] = useState('')

  class CustomCanvas extends ParentCanvas {
    fill([x, y, color]: [number, number, { r: number; g: number; b: number }]) {
      let cell = this.grid[y * this.cols + x]
      console.log(cell)
      if (cell) {
        cell.r = color.r
        cell.g = color.g
        cell.b = color.b
      }
    }
  }

  const run = () => {
    const lexer = new Lexer(code)
    const parser = new Parser(lexer.tokens)
    const interpreter = new Interpreter()
    interpreter.run(parser.ast, {
      ...stdlib,
      Canvas: CustomCanvas
    })
  }

  return (
    <div
      className={styles.editor}
      style={{ borderTop: '1px solid var(--border)' }}>
      <div className={styles.editable}>
        <CodeMirror
          height="100%"
          theme={quietlight}
          onChange={value => {
            setCode(value)
          }}
        />
      </div>
      <div>
        <div className={styles.tabs}>
          <div
            className={styles.tab}
            onClick={() => {
              run()
            }}>
            Run
          </div>
        </div>
        <Easel />
      </div>
    </div>
  )
}
