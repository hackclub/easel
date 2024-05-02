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
import LexerParserTransform from '@/components/interactive/LexerParserTransform'
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
import { useEffect, useRef } from 'react'

const trim = (str, chars) => str.split(chars).filter(Boolean).join(chars)

const Mermaid = dynamic(() => import('@/components/Mermaid'), { ssr: false })
const Editor = dynamic(() => import('@/components/Editor'), { ssr: false })
const Runtime = dynamic(() => import('@/components/Runtime'), { ssr: false })

const components = {
  Canvas,
  Lexer,
  Mermaid,
  LexerParserTransform,
  Node,
  Editor: props => {
    return (
      <>
        {props.children}
        <Editor {...props} />
      </>
    )
  },
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
  const curr = parts.findIndex(part => part.title === title)
  const prev = parts[curr - 1]
  const next = parts[curr + 1]

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
              <Link href={part.slug}>{part.title}</Link>
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
        <MDXRemote {...page} components={components} />
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
        <footer></footer>
      </div>
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
