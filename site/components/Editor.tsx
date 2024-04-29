import CodeMirror from '@uiw/react-codemirror'
import { javascript } from '@codemirror/lang-javascript'
import styles from './Editor.module.scss'
import { useEffect, useState } from 'react'
import { quietlight } from '@uiw/codemirror-theme-quietlight'

export function Code({
  tab,
  value = '// Start typing to get started!'
}: {
  tab: string
  value?: string
}) {
  return (
    <CodeMirror
      value={value}
      height="50vh"
      theme={quietlight}
      extensions={[javascript({ jsx: true })]}
      onChange={value => {
        console.log(value)
        localStorage.setItem(tab, value)
      }}
    />
  )
}

export default function Editor({
  initialTab = 'easel.js'
}: {
  initialTab?: string
}) {
  const [activeTab, setActiveTab] = useState(initialTab)
  const [tabs, setTabs] = useState<{ [key: string]: string }>({
    'ast.js': '',
    'easel.js': `import fs from 'fs'
import readline from 'node:readline'
import { Lexer } from './lexer.js'
import { Parser } from './parser.js'
import { Interpreter } from './interpreter.js'
import stdlib, { EaselError } from './stdlib.js'

const readFile = location =>
  new Promise((resolve, reject) =>
    fs.readFile(location, 'utf-8', (err, data) => {
      if (err) return reject(err)
      resolve(data.toString())
    })
  )

const writeFile = (location, data) =>
  new Promise((resolve, reject) =>
    fs.writeFile(location, data, err => {
      if (err) return reject(err)
      resolve()
    })
  )

;(async () => {
  let argv = process.argv.slice(2)
  const debug = argv.find(cmd => cmd === '--dbg') ? true : false
  argv = argv.filter(arg => arg !== '--dbg')

  const location = argv[0]
  if (location) {
    const program = await readFile(location)

    const lexer = new Lexer(program)
    try {
      lexer.scanTokens()
    } catch (err) {
      if (err instanceof EaselError) {
        console.log(err)
        process.exit(1)
      }
    } finally {
      if (debug) await writeFile('tokens.json', JSON.stringify(lexer.tokens))
    }

    const parser = new Parser(lexer.tokens)
    try {
      parser.parse()
    } catch (err) {
      if (err instanceof EaselError) {
        console.log(err)
        process.exit(2)
      }
    } finally {
      if (debug) await writeFile('ast.json', JSON.stringify(parser.ast))
    }

    const interpreter = new Interpreter()
    try {
      interpreter.run(parser.ast, stdlib)
    } catch (err) {
      if (err instanceof EaselError) console.log(err.toString())
    }
  } else {
    // Interactive REPL
    const interpreter = new Interpreter()
    let scope = {
      ...stdlib,
      exit: () => process.exit(0)
    }

    const input = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    })

    // Remember to close stream before exiting
    process.on('SIGINT', () => {
      input.close()
    })

    const repl = line => {
      let hadError = false

      const lexer = new Lexer(line)
      try {
        lexer.scanTokens()
      } catch (err) {
        if (err instanceof EaselError) {
          hadError = true
          console.log(err.toString())
        }
      }

      if (!hadError) {
        const parser = new Parser(lexer.tokens)
        try {
          parser.parse()
        } catch (err) {
          if (err instanceof EaselError) console.log(err.toString())
        }

        try {
          scope = interpreter.run(parser.ast, scope)
        } catch (err) {
          if (err instanceof EaselError) console.log(err.toString())
        }
      }

      input.question('> ', repl)
    }

    input.question('> ', repl)
  }
})()`,
    'interpreter.js': '',
    'lexer.js': '',
    'parser.js': '',
    'stdlib.js': '',
    'program.easel': ''
  })

  useEffect(() => {
    // Pull from localStorage
    let populated = Object.assign({}, tabs)
    for (let key of Object.keys(populated)) {
      populated[key] = localStorage.getItem(key) || ''
    }
    setTabs(populated)
  }, [])

  useEffect(() => {
    setTabs(old => {
      return {
        ...old,
        [activeTab]: localStorage.getItem(activeTab) || ''
      }
    })
  }, [activeTab])

  return (
    <div className={styles.editor}>
      <div className={styles.editable}>
        <div className={styles.tabs}>
          {Object.keys(tabs).map(tab => (
            <div
              className={styles.tab}
              key={tab}
              style={{
                backgroundColor:
                  activeTab === tab ? 'transparent' : 'var(--background)'
              }}
              onClick={() => setActiveTab(tab)}>
              {tab}
            </div>
          ))}
        </div>
        <Code tab={activeTab} value={tabs[activeTab]} />
      </div>
      <div className={styles.output}>
        <div className={styles.tabs}>
          <div
            className={styles.tab}
            style={{ backgroundColor: 'var(--background)' }}>
            Output
          </div>
          <div className={styles.tab}>Easel</div>
          <div className={styles.tab} style={{ justifySelf: 'flex-end' }}>
            Run
          </div>
        </div>
      </div>
    </div>
  )
}
