import CodeMirror, { ReactCodeMirrorRef } from '@uiw/react-codemirror'
import { javascript } from '@codemirror/lang-javascript'
import styles from './Editor.module.scss'
import { useEffect, useState, useRef, MutableRefObject } from 'react'
import { quietlight } from '@uiw/codemirror-theme-quietlight'
import { Nodebox, type ShellProcess } from '@codesandbox/nodebox'
import { trim } from './trim'

declare global {
  interface Window {
    nodebox: Nodebox
    shells: ShellProcess[]
  }
}

export function Output({ code }: { code: string }) {
  console.log(code)
  return (
    <CodeMirror
      value={code}
      minHeight="50vh"
      height="50vh"
      theme={quietlight}
      readOnly={true}
      extensions={[javascript()]}
    />
  )
}

export function Code({
  tab,
  value = '// Start typing to get started!',
  setValue
}: {
  tab: string
  value?: string
  setValue: (value: string) => any
}) {
  return (
    <CodeMirror
      value={value}
      minHeight="50vh"
      maxHeight="50vh"
      height="50vh"
      theme={quietlight}
      extensions={tab.endsWith('.js') ? [javascript({ jsx: true })] : []}
      onChange={value => {
        setValue(value)
      }}
      onFocus={event => {
        setValue(localStorage.getItem(tab) || '')
      }}
    />
  )
}

function Loading() {
  const codeRef = useRef<HTMLElement | null>(null)

  useEffect(() => {
    // Typing effect!
    const interval = setInterval(() => {
      if (codeRef.current) {
        codeRef.current.innerHTML = codeRef.current.innerHTML + '-'
      }
    }, 10)

    return () => {
      clearInterval(interval)
      if (codeRef.current) codeRef.current.innerHTML = ''
    }
  }, [])

  return <code ref={codeRef}>Loading </code>
}

export default function Editor({
  initialTab = 'easel.js',
  easelFile = 'program.easel',
  initialTabs,
  setInitialTabs
}: {
  initialTab?: string
  easelFile?: string
  initialTabs: { [key: string]: string }
  setInitialTabs: (old: any) => any
}) {
  const [tabs, setTabs] = useState(Object.assign({}, initialTabs))
  const [loading, setLoading] = useState(false)
  const [outputTabs, setOutputTabs] = useState<{ [key: string]: string }>({
    'tokens.json': '',
    'ast.json': ''
  })
  const [output, setOutput] = useState<Array<{ type: string; value: string }>>(
    []
  )
  const [activeTab, setActiveTab] = useState<string>(initialTab)
  const [activeOutput, setActiveOutput] = useState('Output')

  const run = async () => {
    const nodeIframe = document.getElementById('node-iframe')
    if (nodeIframe) {
      setOutput([])
      setLoading(true)
      const runtime = window.nodebox

      // Update files
      for (let key of Object.keys(tabs)) {
        await runtime.fs.writeFile(key, tabs[key])
      }

      for (let shell of window.shells) shell.kill()
      window.shells = []

      const shell = runtime.shell.create()
      const idx = window.shells.push(shell) - 1

      try {
        await shell.runCommand('node', ['index.js', easelFile, '--dbg'])
        runtime.fs.watch(Object.keys(outputTabs), [], async event => {
          // @ts-expect-error
          const path = trim(event.path, '/')
          const content = await runtime.fs.readFile(path, 'utf-8')
          setOutputTabs(old => ({
            ...old,
            [path]: content
          }))
        })
        shell.stdout.on('data', async (data: string) => {
          // Update output files
          setOutput(old => [
            ...old,
            ...data.split('\n').map(line => ({
              type: data.startsWith('Error') ? 'stderr' : 'stdout',
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
        shell.on('exit', () => {
          window.shells.splice(idx, 1)
        })
      } catch (err) {
        console.log(err)
      } finally {
        setLoading(false)
      }
    }
  }

  return (
    <div className={styles.editor}>
      <div className={styles.editable}>
        <Code
          tab={activeTab}
          value={tabs[activeTab]}
          setValue={(value: string) => {
            setTabs(old => {
              localStorage.setItem(activeTab, value)
              return {
                ...old,
                [activeTab]: value
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
              onClick={() => {
                setActiveTab(tab)
                setTabs(old => ({
                  ...old,
                  [activeTab]: localStorage.getItem(activeTab) || ''
                }))
              }}>
              {tab}
            </div>
          ))}
        </div>
      </div>
      <div className={styles.output}>
        {activeOutput === 'Output' ? (
          <div className={styles.terminal}>
            {loading === true && <Loading />}
            {output.length > 0 &&
              output.map((line, idx) => (
                <code
                  key={idx}
                  style={{ color: line.type === 'stdout' ? 'inherit' : 'red' }}>
                  {line.value}
                </code>
              ))}
          </div>
        ) : (
          <Output code={outputTabs[activeOutput]} />
        )}
        <div className={styles.tabs}>
          <div
            className={styles.tab}
            style={{
              backgroundColor:
                activeOutput === 'Output' ? 'var(--background)' : 'transparent'
            }}
            onClick={() => setActiveOutput('Output')}>
            Output
          </div>
          <div className={styles.tab}>Easel</div>
          <div
            className={styles.tab}
            style={{ alignSelf: 'flex-end' }}
            onClick={run}>
            Run
          </div>
          <div
            style={{
              display: 'flex',
              flex: 1,
              justifyContent: 'flex-end'
            }}>
            {Object.keys(outputTabs)
              .filter(tab => outputTabs[tab].length > 0)
              .map((tab, idx) => (
                <div
                  className={styles.tab}
                  key={tab}
                  style={{
                    borderLeft: idx === 0 ? '1px solid var(--border)' : 'none',
                    backgroundColor:
                      activeOutput === tab ? 'var(--background)' : 'transparent'
                  }}
                  onClick={() => setActiveOutput(tab)}>
                  {tab}
                </div>
              ))}
          </div>
        </div>
      </div>
    </div>
  )
}
