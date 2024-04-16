import fs from 'fs'
import path from 'path'
import { serialize } from 'next-mdx-remote/serialize'
import Link from 'next/link'
import styles from '../styles/Landing.module.scss'

export default function Index({
  parts
}: {
  parts: Array<{ title: string; slug: string }>
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
      <main className={styles.header}>
        <h1>Write a programming language, get a terminal in a box!</h1>
      </main>
    </>
  )
}

export async function getServerSideProps() {
  const readdir = (dir: string) =>
    fs.readdirSync(dir, { withFileTypes: true }).map(dirent => dirent.name)

  const parts = readdir('content')
  let titles = []
  for (let part of parts) {
    const content = fs.readFileSync(path.join('content', part), 'utf-8')
    const serialized = await serialize(content, { parseFrontmatter: true })
    titles.push({
      order: serialized.frontmatter.order,
      title: serialized.frontmatter.title,
      slug: `/${part.replace('.mdx', '')}`
    })
  }

  return {
    props: {
      parts: titles.sort((a, b) => (Number(a.order) > Number(b.order) ? 1 : -1))
    }
  }
}
