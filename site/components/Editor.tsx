import CodeMirror, { ReactCodeMirrorRef } from '@uiw/react-codemirror'
import { javascript } from '@codemirror/lang-javascript'
import { html } from '@codemirror/lang-html'
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

export function Output({
  code,
  height = '50vh'
}: {
  code: string
  height?: string
}) {
  return (
    <CodeMirror
      value={code}
      minHeight={height}
      height={height}
      theme={quietlight}
      readOnly={true}
      extensions={[javascript()]}
    />
  )
}

export function Code({
  tab,
  value = '// Start typing to get started!',
  setValue,
  height = '50vh'
}: {
  tab: string
  value?: string
  setValue: (value: string) => any
  height?: string
}) {
  return (
    <CodeMirror
      value={value}
      minHeight={height}
      maxHeight={height}
      height={height}
      theme={quietlight}
      extensions={
        tab.endsWith('.js')
          ? [javascript({ jsx: true })]
          : tab.endsWith('.html')
          ? [html()]
          : []
      }
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
  const gridRef = useRef<HTMLDivElement>(null)
  const previewIframe = useRef<HTMLIFrameElement>(null)
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
  const [height, setHeight] = useState('1px')

  useEffect(() => {
    if (gridRef.current) {
      const grid = gridRef.current
      setHeight(`${grid.parentElement?.offsetWidth}px`)
    }
  }, [])

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
        const command = await shell.runCommand('node', [
          'index.js',
          easelFile,
          '--dbg'
        ])

        if (previewIframe.current) {
          const { url } = await runtime.preview.getByShellId(command.id)
          previewIframe.current.setAttribute('src', url)
        }

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
          height={height}
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
        <div ref={gridRef} style={{ height }}>
          {activeOutput === 'Output' ? (
            <div className={styles.terminal}>
              {loading === true && <Loading />}
              {output.length > 0 &&
                output.map((line, idx) => (
                  <code
                    key={idx}
                    style={{
                      color: line.type === 'stdout' ? 'inherit' : 'red'
                    }}>
                    {line.value}
                  </code>
                ))}
            </div>
          ) : (
            activeOutput !== 'Easel' && (
              <Output height={height} code={outputTabs[activeOutput]} />
            )
          )}
          <iframe
            ref={previewIframe}
            style={{
              height,
              display: activeOutput === 'Easel' ? 'block' : 'none'
            }}
          />
        </div>
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
          <div
            className={styles.tab}
            style={{
              backgroundColor:
                activeOutput === 'Easel' ? 'var(--background)' : 'transparent'
            }}
            onClick={() => setActiveOutput('Easel')}>
            Easel
          </div>
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
