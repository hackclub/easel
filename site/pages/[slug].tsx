import fs from 'fs'
import { MDXRemote, MDXRemoteSerializeResult } from 'next-mdx-remote'
import { serialize } from 'next-mdx-remote/serialize'
import Link from 'next/link'
import { remark } from 'remark'
import remarkToc from 'remark-toc'
import 'highlight.js/styles/base16/solarized-light.min.css'
import rehypeHighlight from 'rehype-highlight'
import Canvas from '@/components/Canvas'
import Lexer from '@/components/interactive/Lexer'
import dynamic from 'next/dynamic'
import path from 'path'
import Head from 'next/head'
import Meta from '@hackclub/meta'
import Node from '@/components/interactive/Node'
import rehypeSlug from 'rehype-slug'
import { rehype } from 'rehype'
import { Demo } from '@/components/Interpreter'
import styles from '@/styles/Part.module.scss'
import Confetti from 'react-canvas-confetti'
import { useRef, useEffect, useState } from 'react'
import { trim } from '@/components/trim'
import Icon from '@hackclub/icons'

const Mermaid = dynamic(() => import('@/components/Mermaid'), { ssr: false })
const Editor = dynamic(() => import('@/components/Editor'), { ssr: false })
const Runtime = dynamic(() => import('@/components/Runtime'), { ssr: false })

const components = {
  Canvas,
  Lexer,
  Mermaid,
  Node,
  pre: props => {
    return (
      <div className="pre-wrapper">
        <pre>{props.children}</pre>
      </div>
    )
  },
  Demo,
  Celebrate: () => {
    return (
      <>
        <div className={styles.celebrate}>
          <img src="https://github.com/hackclub/dinosaurs/raw/main/party_orpheus.png" />
          <button>Celebrate with Orpheus</button>
        </div>
        <Confetti className={styles.confetti} width={200} height={200} />
      </>
    )
  }
}

export default function Index({
  parts,
  title,
  page,
  toc
}: {
  parts: Array<{ title: string; slug: string }>
  title: string
  page: MDXRemoteSerializeResult
  toc: MDXRemoteSerializeResult
}) {
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
  const curr = parts.findIndex(part => part.title === title)
  const prev = parts[curr - 1]
  const next = parts[curr + 1]

  useEffect(() => {
    // Pull from localStorage
    let populated = Object.assign({}, tabs)
    for (let key of Object.keys(populated)) {
      populated[key] = localStorage.getItem(key) || ''
    }
    setTabs(populated)
  }, [])

  return (
    <>
      <Runtime />
      <Meta
        as={Head}
        title={`${title}`}
        name="Orpheus' Hacky Guide to Writing a Programming Language"
        description="Learn how to write a programming language with Orpheus the dino and me, the narrator!"
        image="/cartoons/wizard.png"
        color="#ec3750"
      />
      <header>
        <div>
          <img
            id="logo"
            src="https://assets.hackclub.com/flag-orpheus-top.svg"
          />
          <Link href="/">
            <h1>orpheus' hacky guide to writing a programming language</h1>
          </Link>
        </div>
        <div>
          <h2>chapters</h2>
          {parts.map((part, idx) => (
            <p key={idx}>
              <a href={part.slug}>{part.title}</a>
            </p>
          ))}
        </div>
      </header>
      <section className="prose" style={{ marginBottom: '2em' }}>
        <p>
          High schooler?{' '}
          <a href="https://hackclub.com" target="_blank">
            Hack Club
          </a>{' '}
          is running a{' '}
          <a href="https://github.com/hackclub/langjam" target="_blank">
            programming language jam
          </a>
          . Build a fun programming language with friends, get a
          terminal-in-a-box to run it on!
        </p>
      </section>
      <div className="prose">
        <div className="toc">
          <MDXRemote {...toc} />
        </div>
        <h1>{title}</h1>
        <MDXRemote
          {...page}
          components={{
            ...components,
            Editor: props => {
              return (
                <>
                  {props.children}
                  <Editor
                    {...props}
                    initialTabs={tabs}
                    setInitialTabs={setTabs}
                  />
                </>
              )
            }
          }}
        />
        <div className="pagination">
          {prev ? (
            <Link href={prev.slug}>
              <div>
                <p className="hint">&larr; Previous</p>
                <p>{prev.title}</p>
              </div>
            </Link>
          ) : (
            <div style={{ minWidth: '50%' }} />
          )}
          {next && (
            <Link href={next.slug}>
              <div>
                <p className="hint">Next &rarr;</p>
                <p>{next.title}</p>
              </div>
            </Link>
          )}
        </div>
      </div>
      <footer>
        <div className="prose sm">
          <h2>A project by Hack Club.</h2>
          <p>
            Hack Club is a registered 501(c)3 nonprofit organization that
            supports a network of 20k+ technical high schoolers. We believe you
            learn best by building when you're learning and shipping technical
            projects with your friends, so we've started You Ship, We Ship, a
            program where you ship a technical project and we ship you something
            in exchange. In the past few years, we{' '}
            <a href="https://hackclub.com/onboard">
              fabricated custom PCBs designed by 265 teenagers
            </a>
            ,{' '}
            <a href="https://github.com/hackclub/the-hacker-zephyr">
              hosted the world's longest hackathon on land
            </a>
            , and{' '}
            <a href="https://hackclub.com/winter">gave away $75k of hardware</a>
            .
          </p>
          <div className="footer">
            <div>
              <h3>Hack Club</h3>
              <p>
                <a href="https://hackclub.com/philosophy">Philosophy</a>
              </p>
              <p>
                <a href="https://hackclub.com/team">Our Team & Board</a>
              </p>
              <p>
                <a href="https://hackclub.com/jobs">Jobs</a>
              </p>
              <p>
                <a href="https://hackclub.com/brand">Branding</a>
              </p>
              <p>
                <a href="https://hackclub.com/press">Press Inquiries</a>
              </p>
              <p>
                <a href="https://hackclub.com/donate">Donate</a>
              </p>
            </div>
            <div>
              <h3>Resources</h3>
              <p>
                <a href="https://hackclub.com/community">Community</a>
              </p>
              <p>
                <a href="https://hackclub.com/onboard">OnBoard</a>
              </p>
              <p>
                <a href="https://sprig.hackclub.com">Sprig</a>
              </p>
              <p>
                <a href="https://blot.hackclub.com">Blot</a>
              </p>
              <p>
                <a href="https://hackclub.com/bin">Bin</a>
              </p>
              <p>
                <a href="https://jams.hackclub.com">Jams</a>
              </p>
            </div>
          </div>
          <p>
            © {new Date().getFullYear()} Hack Club. 501(c)(3) nonprofit (EIN:
            81-2908499)
          </p>
        </div>
      </footer>
    </>
  )
}

export async function getStaticPaths() {
  const readdir = (dir: string) =>
    fs.readdirSync(dir, { withFileTypes: true }).map(dirent => dirent.name)

  const parts = readdir(path.resolve(process.cwd(), 'content'))
  return {
    paths: parts.map(slug => ({ params: { slug: trim(slug, '.mdx') } })),
    fallback: false
  }
}

export async function getStaticProps({ params }: { params: { slug: string } }) {
  const { slug } = params
  let page, title

  // Get parts
  const readdir = (dir: string) =>
    fs.readdirSync(dir, { withFileTypes: true }).map(dirent => dirent.name)

  const parts = readdir(path.resolve(process.cwd(), 'content'))
  let titles = []
  for (let part of parts) {
    const content = fs.readFileSync(path.join('content', part), 'utf-8')
    const serialized = await serialize(content, { parseFrontmatter: true })
    if (part.replace('.mdx', '') === slug) {
      page = content
      title = serialized.frontmatter.title
    }
    titles.push({
      order: serialized.frontmatter.order,
      title: serialized.frontmatter.title,
      slug: `/${part.replace('.mdx', '')}`
    })
  }

  if (!page) return { notFound: true }

  const generateToc = async (content: string) => {
    const headings = [
      '### Table of contents',
      ...content.split('\n').filter(x => /^[#]{1,6} /.test(x))
    ].join('\n')
    const slugger = rehype()
      .data('settings', { fragment: true })
      .use(rehypeSlug)
    const toc = String(await remark().use(remarkToc).process(headings))
    let result: string[] = toc.split('\n').filter(x => x.startsWith('#'))
    if (result.length === 1) result.push('\nNo headings.')
    else
      result = await Promise.all(
        result.map(async (heading, idx) => {
          if (idx === 0) return heading
          const processed = String(
            await slugger.process(`<h1>${trim(heading, '#').trim()}</h1>`)
          )
          return `[${trim(heading, '#').trim()}](#${trim(
            processed.match(/"\S+"/)[0],
            '"'
          )})\n`
        })
      )
    return result.join('\n')
  }

  return {
    props: {
      parts: titles.sort((a, b) =>
        Number(a.order) > Number(b.order) ? 1 : -1
      ),
      page: await serialize(page, {
        parseFrontmatter: true,
        mdxOptions: {
          rehypePlugins: [rehypeSlug, rehypeHighlight]
        }
      }),
      toc: await serialize(await generateToc(page)),
      title
    }
  }
}
