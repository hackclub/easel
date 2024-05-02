import CodeMirror from '@uiw/react-codemirror'
import { javascript } from '@codemirror/lang-javascript'
import styles from './Editor.module.scss'
import { useEffect, useState, useRef } from 'react'
import { quietlight } from '@uiw/codemirror-theme-quietlight'
import { Nodebox } from '@codesandbox/nodebox'
import { loadRuntime } from './Runtime'

declare global {
  interface Window {
    nodebox: Nodebox
  }
}

export function Code({
  tab,
  value = '// Start typing to get started!',
  setValue
}: {
  tab: string
  value?: string
  setValue: () => any
}) {
  return (
    <CodeMirror
      value={value}
      minHeight="50vh"
      height="50vh"
      theme={quietlight}
      extensions={tab.endsWith('.js') ? [javascript({ jsx: true })] : []}
      onChange={value => {
        localStorage.setItem(tab, value)
      }}
      onFocus={() => setValue()}
    />
  )
}

export default function Editor({
  initialTab = 'easel.js'
}: {
  initialTab?: string
}) {
  const [output, setOutput] = useState<Array<{ type: string; value: string }>>(
    []
  )
  const [activeTab, setActiveTab] = useState(initialTab)
  const [tabs, setTabs] = useState<{ [key: string]: string }>({
    'ast.js': '',
    'easel.js': '',
    'interpreter.js': '',
    'lexer.js': '',
    'parser.js': '',
    'stdlib.js': '',
    'program.easel': '',
    'test.easel': ''
  })

  const previewIframe = useRef<HTMLIFrameElement | null>(null)

  useEffect(() => {
    // Pull from localStorage
    let populated = Object.assign({}, tabs)
    for (let key of Object.keys(populated)) {
      populated[key] = localStorage.getItem(key) || ''
    }
    setTabs(populated)
  }, [])

  const run = async () => {
    const nodeIframe = document.getElementById('node-iframe')
    if (nodeIframe) {
      setOutput([])
      const runtime = window.nodebox

      // Update files
      for (let key of Object.keys(tabs)) {
        await runtime.fs.writeFile(key, tabs[key])
        console.log(await runtime.fs.readFile(key, 'utf-8'))
      }

      // Create shell process
      const shell = runtime.shell.create()

      // Run node
      const nextProcess = await shell.runCommand('node', [
        'index.js',
        'program.easel',
        '--dbg'
      ])

      // Upload to preview
      const previewInfo = await runtime.preview.getByShellId(nextProcess.id)
      shell.stdout.on('data', async (data: string) => {
        setOutput(old => [
          ...old,
          ...data.split('\n').map(line => ({
            type: 'stdout',
            value: line
          }))
        ])
      })
      shell.stderr.on('data', (data: string) => {
        setOutput(old => [
          ...old,
          ...data.split('\n').map(line => ({
            type: 'stderr',
            value: line
          }))
        ])
      })
    }
  }

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
        <Code
          tab={activeTab}
          value={tabs[activeTab]}
          setValue={() => {
            setTabs(old => {
              return {
                ...old,
                [activeTab]: localStorage.getItem(activeTab) || ''
              }
            })
          }}
        />
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
      </div>
      <div className={styles.output}>
        <div className={styles.terminal}>
          {output.length ? (
            output.map((line, idx) => (
              <code
                key={idx}
                style={{ color: line.type === 'stdout' ? 'inherit' : 'red' }}>
                {line.value}
              </code>
            ))
          ) : (
            <code>
              <i>Output will show up here.</i>
            </code>
          )}
        </div>
        <div className={styles.tabs}>
          <div
            className={styles.tab}
            style={{ backgroundColor: 'var(--background)' }}>
            Output
          </div>
          <div className={styles.tab}>Easel</div>
          <div
            className={styles.tab}
            style={{ alignSelf: 'flex-end' }}
            onClick={run}>
            Run
          </div>
        </div>
      </div>
    </div>
  )
}
