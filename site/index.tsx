import styles from '@/styles/Landing.module.scss'
import Link from 'next/link'
import Meta from '@hackclub/meta'
import Head from 'next/head'
import fs from 'fs'
import path from 'path'
import { serialize } from 'next-mdx-remote/serialize'

export default function Index({
  parts
}: {
  parts: Array<{ title: string; slug: string }>
}) {
  return (
    <div>
      <Meta
        as={Head}
        title="Orpheus's Hacky Guide to Writing a Programming Language"
        description="Learn how to write a programming language with Orpheus the dino and me, the narrator!"
        image="https://cloud-bimy66myq-hack-club-bot.vercel.app/0video-capture-8135_2-min.png"
        color="#ec3750"
      />
      <main className={styles.main}>
        <h1>Write a programming lang, get a terminal in a box!</h1>
        {/* <div className={styles.buttons}>
          <Link href="/">
            <div>I don't know how to write one</div>
          </Link>
          <Link href="/">
            <div>I just wrote one!</div>
          </Link>
        </div> */}
      </main>
      {/* <footer className={styles.footer}>
        <p>
          Hack Club is a registered 501(c)3 nonprofit organization that supports
          a network of 20k+ technical high schoolers. We believe you learn best
          by building so we're removing barriers to hardware access so any
          teenager can explore. In the past few years, we
        </p>
      </footer> */}
    </div>
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
