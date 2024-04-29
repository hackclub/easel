import styles from '@/styles/Landing.module.scss'
import Link from 'next/link'
import Meta from '@hackclub/meta'
import Head from 'next/head'

export default function Index() {
  return (
    <div>
      <Meta as={Head} title={'Hack Club'} />
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
