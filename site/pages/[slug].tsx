import fs from 'fs'
import { MDXRemote, MDXRemoteSerializeResult } from 'next-mdx-remote'
import { serialize } from 'next-mdx-remote/serialize'
import Link from 'next/link'
import path from 'path'
import { remark } from 'remark'
import remarkToc from 'remark-toc'
import 'highlight.js/styles/base16/solarized-light.min.css'
import rehypeHighlight from 'rehype-highlight'

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
  return (
    <>
      <header>
        <div>
          <img
            id="logo"
            src="https://assets.hackclub.com/flag-orpheus-top.svg"
          />
          <h1>orpheus' hacky guide to writing a programming language</h1>
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
          High schooler? <a href="https://hackclub.com">Hack Club</a> is running
          a <a href="#">programming language jam</a>. Build a fun programming
          language with friends, get a terminal-in-a-box to run it on!
        </p>
      </section>
      <div className="prose">
        <div className="toc">
          <MDXRemote {...toc} />
        </div>
        <h1>{title}</h1>
        <MDXRemote {...page} />
        <div className="pagination">
          <a href="#">
            <div>
              <p className="hint">&larr; Previous</p>
              <p>Orpheus finds an easel in the mail</p>
            </div>
          </a>
          <a href="#">
            <div>
              <p className="hint">Next &rarr;</p>
              <p>Orpheus writes a parser</p>
            </div>
          </a>
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

  const parts = readdir('content')
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
