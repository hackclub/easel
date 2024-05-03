import CodeMirror from '@uiw/react-codemirror'
import { javascript } from '@codemirror/lang-javascript'
import styles from './Editor.module.scss'
import { useEffect, useState, useRef } from 'react'
import { quietlight } from '@uiw/codemirror-theme-quietlight'
import { Nodebox, type ShellProcess } from '@codesandbox/nodebox'
import { loadRuntime } from './Runtime'

declare global {
  interface Window {
    nodebox: Nodebox
    shells: ShellProcess[]
  }
}

export function Output({ code }: { code: string }) {
  return (
    <CodeMirror
      value={code}
      minHeight="50vh"
      height="50vh"
      theme={quietlight}
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
  setValue: () => any
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
        localStorage.setItem(tab, value)
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
  tabs,
  setTabs
}: {
  initialTab?: string
  easelFile?: string
  tabs: { [key: string]: string }
  setTabs: (old: any) => any
}) {
  const [loading, setLoading] = useState(false)
  const [output, setOutput] = useState<Array<{ type: string; value: string }>>(
    []
  )
  const [activeTab, setActiveTab] = useState(initialTab)

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
