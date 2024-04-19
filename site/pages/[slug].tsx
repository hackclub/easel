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

const Mermaid = dynamic(() => import('@/components/Mermaid'), { ssr: false })

const components = { Canvas, Lexer, Mermaid, LexerParserTransform, Node }

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
          <h1>
            orpheus' hacky guide to writing a programming language (beta draft!)
          </h1>
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
      <section className="prose">
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
        {/* <div className="toc">
          <MDXRemote {...toc} />
        </div> */}
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

export async function getServerSideProps({
  params
}: {
  params: { slug: string }
}) {
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
    const toc = String(await remark().use(remarkToc).process(headings))
    let result = toc
      .split('\n')
      .filter(x => x.startsWith('#'))
      .join('\n')
    if (result === '### Table of contents') result += '\nNo headings.'
    return result
  }

  return {
    props: {
      parts: titles.sort((a, b) =>
        Number(a.order) > Number(b.order) ? 1 : -1
      ),
      page: await serialize(page, {
        parseFrontmatter: true,
        mdxOptions: {
          rehypePlugins: [rehypeHighlight]
        }
      }),
      toc: await serialize(await generateToc(page)),
      title
    }
  }
}
