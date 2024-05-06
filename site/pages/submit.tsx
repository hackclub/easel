import Meta from '@hackclub/meta'
import Head from 'next/head'
import Link from 'next/link'
import fs from 'fs'
import path from 'path'
import { serialize } from 'next-mdx-remote/serialize'
import styles from '@/styles/Submit.module.scss'

export default function Submit({
  parts
}: {
  parts: Array<{ title: string; slug: string }>
}) {
  return (
    <>
      <Meta
        as={Head}
        title={`Submit | Orpheus' Hacky Guide to Writing a Programming Language`}
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
      <main className={styles.form}>
        <div className="prose">
          <h1>
            So, I heard you wrote a programming language! That's awesome. Here's
            some fudge (currently) in exchange.
          </h1>
          <p>First up, let's confirm a few things:</p>
          <ul className={styles.criteria}>
            <li>You're a high schooler (or younger)</li>
            <li>
              This programming language was made recently (since May 01, 2024)
            </li>
            <li>
              Your programming language meets the following criteria:
              <ul>
                <li>Has variables</li>
              </ul>
            </li>
          </ul>
          <hr />
          <form>
            <h2>You</h2>
            <div>
              <label>Name</label>
              <input type="text" name="name" placeholder="Name" />
            </div>
            <div>
              <label>Address</label>
              <label>So we can ship you fudge!</label>
              <input type="text" name="address" />
            </div>
            <div>
              <label>Address</label>
            </div>
          </form>
        </div>
      </main>
    </>
  )
}

export async function getStaticProps() {
  const readdir = (dir: string) =>
    fs.readdirSync(dir, { withFileTypes: true }).map(dirent => dirent.name)

  const parts = readdir(path.resolve(process.cwd(), 'content'))
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
