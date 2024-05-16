import Meta from '@hackclub/meta'
import Head from 'next/head'
import Link from 'next/link'
import fs from 'fs'
import path from 'path'
import { serialize } from 'next-mdx-remote/serialize'
import styles from '@/styles/Submit.module.scss'
import { FormEvent, useState } from 'react'
import invalidBirthdate from '@/components/invalidBirthdate'
import toast from 'react-hot-toast'

export default function Stickers({
  parts
}: {
  parts: Array<{ title: string; slug: string }>
}) {
  const [submitted, setSubmitted] = useState<boolean>(false)

  const submit = async (event: FormEvent) => {
    event.preventDefault()

    const data = {
      email: {
        required: true,
        value: event.target.email.value
      },
      firstname: {
        required: true,
        value: event.target.firstname.value
      },
      lastname: {
        required: true,
        value: event.target.lastname.value
      },
      address: {
        required: true,
        value: event.target.address.value
      },
      city: {
        required: true,
        value: event.target.city.value
      },
      state: {
        required: true,
        value: event.target.state.value
      },
      zip: {
        required: true,
        value: event.target.zip.value
      },
      country: {
        required: true,
        value: event.target.country.value
      },
      address2: {
        value: event.target.address2.value
      },
      birthdate: {
        value: event.target.birthdate.value
      }
    }

    let submission = {}
    for (let [key, value] of Object.entries(data)) {
      if (value.required && !value.value) {
        toast.error('Make sure you fill out all the fields!')
      }
      submission[key] = value.value
    }

    if (invalidBirthdate(submission.birthdate)) submission.highschool = false
    else submission.highschool = true

    fetch('/api/stickers', {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(submission)
    })
      .then(res => res.json())
      .then(res => {
        setSubmitted(true)
      })
      .catch(err => {
        toast.error(err.toString())
      })
  }

  return (
    <>
      <style jsx global>{`
        body {
          background-color: var(--background);
        }
      `}</style>
      <Meta
        as={Head}
        title={`Stickers | Orpheus' Hacky Guide to Writing a Programming Language`}
        description="Learn how to write a programming language in a weekend!"
        image="/cartoons/site.png"
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
          <h1 style={{ marginTop: '1em' }}>
            Sign up for Hack Club's mailing list! And if you're a teenager,
            we'll send you some custom stickers, like{' '}
            <a href="/orpheus-writes-interpreter#wizard-orpheus">
              Wizard Orpheus
            </a>
            .
          </h1>
          {submitted === false ? (
            <>
              <form onSubmit={submit}>
                <div>
                  <label className={styles.required}>Email</label>
                  <input
                    type="email"
                    name="email"
                    required
                    autoComplete="off"
                  />
                </div>
                <div className={styles.flex}>
                  <div>
                    <label className={styles.required}>First Name</label>
                    <input
                      type="text"
                      name="firstname"
                      required
                      placeholder="First name"
                      autoComplete="off"
                    />
                  </div>
                  <div>
                    <label className={styles.required}>Last Name</label>
                    <input
                      type="text"
                      name="lastname"
                      required
                      placeholder="Last name"
                      autoComplete="off"
                    />
                  </div>
                  <div>
                    <label className={styles.required}>Birthdate</label>
                    <input type="date" name="birthdate" required />
                  </div>
                </div>
                <div>
                  <label className={styles.required}>Address</label>
                  <input
                    type="text"
                    name="address"
                    required
                    placeholder="15 Falls Road"
                    autoComplete="off"
                  />
                </div>
                <div>
                  <label>Address line 2</label>
                  <input
                    type="text"
                    name="address2"
                    placeholder="APT #2"
                    autoComplete="off"
                  />
                </div>
                <div className={styles.flex}>
                  <div>
                    <label className={styles.required}>City</label>
                    <input
                      type="text"
                      name="city"
                      required
                      placeholder="Shelburne"
                      autoComplete="off"
                    />
                  </div>
                  <div>
                    <label className={styles.required}>State</label>
                    <input
                      type="text"
                      name="state"
                      required
                      placeholder="VT"
                      autoComplete="off"
                    />
                  </div>
                  <div>
                    <label className={styles.required}>Zip code</label>
                    <input
                      autoComplete="off"
                      type="text"
                      required
                      name="zip"
                      placeholder="05482"
                    />
                  </div>
                  <div>
                    <label className={styles.required}>Country</label>
                    <input
                      autoComplete="off"
                      type="text"
                      required
                      name="country"
                      placeholder="USA"
                    />
                  </div>
                </div>
                <div>
                  <button type="submit">Subscribe</button>
                </div>
              </form>
            </>
          ) : (
            <div>
              <p>
                Awesome! You're the coolest. If you're a high schooler, check
                your mailbox in the coming week!
              </p>
            </div>
          )}
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
