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
  const [height, setHeight] = useState('1px')
  const gridRef = useRef<HTMLDivElement | null>(null)

  class CustomCanvas extends ParentCanvas {
    fill([x, y, color]: [number, number, { r: number; g: number; b: number }]) {
      let cell = this.grid[y * this.cols + x]
      if (cell) {
        cell.r = color.r
        cell.g = color.g
        cell.b = color.b

        // Set actual cell color to that
        if (gridRef.current) {
          const row = gridRef.current.querySelectorAll(':scope > div')[y]
          const col = row.querySelectorAll('div')[x]
          col.style.backgroundColor = `rgb(${color.r}, ${color.g}, ${color.b})`
        }
      }
    }

    erase([x, y]: [number, number]) {
      let cell = this.grid[y * this.cols + x]
      if (gridRef.current && cell) {
        const row = gridRef.current.querySelectorAll(':scope > div')[y]
        const col = row.querySelectorAll('div')[x]
        col.style.backgroundColor = `rgb(241, 241, 241)`
      }
    }
  }

  const run = () => {
    try {
      const lexer = new Lexer(code)
      lexer.scanTokens()
      const parser = new Parser(lexer.tokens)
      parser.parse()
      const interpreter = new Interpreter()
      let scope = interpreter.run(parser.ast, {
        ...stdlib,
        Canvas: new CustomCanvas()
      })
      if (Object.keys(scope).includes('painting')) {
        setInterval(() => {
          const lexer = new Lexer('painting()')
          lexer.scanTokens()
          const parser = new Parser(lexer.tokens)
          parser.parse()
          interpreter.run(parser.ast, scope)
        }, 100)
      }
    } catch (err) {
      console.log(err)
    }
  }

  useEffect(() => {
    if (gridRef.current) {
      const grid = gridRef.current
      const gridWidth = grid.parentElement?.offsetWidth || 500
      grid.style.height = `${gridWidth}px`
      setHeight(`${grid.parentElement?.parentElement?.offsetHeight}px`)

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
    <div className={styles.editor} style={{ borderRight: 'none !important' }}>
      <div
        className={styles.editable}
        style={{ borderTop: '1px solid var(--border)' }}>
        <CodeMirror
          height={height}
          theme={quietlight}
          onChange={value => {
            setCode(value)
          }}
        />
      </div>
      <div>
        <div className={styles.tabs} style={{ justifyContent: 'flex-end' }}>
          <div
            className={styles.tab}
            style={{ borderLeft: '1px solid var' }}
            onClick={() => {
              run()
            }}>
            Run
          </div>
        </div>
        <div className={styles.easel}>
          <div ref={gridRef} />
        </div>
      </div>
    </div>
  )
}
