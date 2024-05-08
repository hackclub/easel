import Meta from '@hackclub/meta'
import Head from 'next/head'
import Link from 'next/link'
import fs from 'fs'
import path from 'path'
import { serialize } from 'next-mdx-remote/serialize'
import styles from '@/styles/Submit.module.scss'
import { FormEvent, useState, useRef } from 'react'

export default function Submit({
  parts
}: {
  parts: Array<{ title: string; slug: string }>
}) {
  const submit = async (event: FormEvent) => {
    event.preventDefault()
    const data = {
      firstname: {
        required: true,
        value: event.target.firstname.value
      },
      lastname: {
        required: true,
        value: event.target.lastname.value
      },
      birthdate: {
        required: true,
        value: event.target.birthdate.value
      },
      email: {
        required: true,
        value: event.target.email.value
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
      id: {
        required: true,
        value: event.target.id.value
      },
      github: {
        required: true,
        value: event.target.github.value
      },
      pr: {
        required: true,
        value: event.target.pr.value
      },
      demo: {
        required: true,
        value: event.target.demo.value
      },
      discovery: {
        required: true,
        value: event.target.discovery.value
      },
      compliments: {
        value: event.target.compliments.value
      },
      improvements: {
        value: event.target.improvements.value
      }
    }

    let submission = {}
    for (let [key, value] of Object.entries(data)) {
      if (value.required && !value.value) {
        console.log(key, value)
        return
      }
      submission[key] = value.value
    }

    fetch('/api/submit', {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(submission)
    })
      .then(res => res.json())
      .then(res => {
        console.log(res)
      })
      .catch(err => console.log(err))
  }

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
          <h1
            style={{
              marginTop: '1em'
            }}>
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
              Your programming language meets the criteria listed{' '}
              <a href="/orpheus-decodes-program">here</a>
            </li>
          </ul>
          <hr />
          <form onSubmit={submit}>
            <h2>You</h2>
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
              <p>So we can ship you fudge!</p>
              <input
                type="text"
                name="address"
                required
                placeholder="15 Falls Road"
                autoComplete="off"
              />
            </div>
            <div>
              <label className={styles.required}>Email</label>
              <input type="email" name="email" required autoComplete="off" />
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
              <label>Address line 2</label>
              <input
                type="text"
                name="address2"
                placeholder="15 Falls Road, Shelburne VT 05482"
                autoComplete="off"
              />
            </div>
            <div>
              <label className={styles.required}>Photo of student ID</label>
              <p>
                We need this to confirm that you are, in fact, a student. Head
                over to the{' '}
                <a href="https://hackclub.com/slack">Hack Club Slack</a>, head
                to{' '}
                <a href="https://hackclub.slack.com/archives/C016DEDUL87">
                  #cdn
                </a>
                , and upload your student ID to get a URL you can paste here.
              </p>
              <input
                type="text"
                name="id"
                required
                autoComplete="off"
                placeholder="https://cloud-80eg2m8id-hack-club-bot.vercel.app/0thinking_rac.png"
              />
            </div>
            <h2>Project</h2>
            <div>
              <label className={styles.required}>GitHub username</label>
              <input
                type="text"
                name="github"
                required
                autoComplete="off"
                placeholder="hackclub"
              />
            </div>
            <div>
              <div>
                <label className={styles.required}>
                  Link to your pull request
                </label>
                <p>
                  Don't know how to make a pull request? Check out{' '}
                  <a href="">our guide</a>.
                </p>
                <input
                  type="text"
                  name="pr"
                  required
                  autoComplete="off"
                  placeholder="https://github.com/hackclub/langjam/issues/3"
                />
              </div>
              <div>
                <label className={styles.required}>
                  Demo of your project on{' '}
                  <a href="https://asciinema.org/">Asciinema</a>
                </label>
                <p>
                  Record a demo using Asciinema! Don't know how to get started?
                  Check out <a href="#">our guide</a>.
                </p>
                <input
                  type="text"
                  name="demo"
                  required
                  autoComplete="off"
                  placeholder="https://asciinema.org/a/590145"
                />
              </div>
            </div>
            <h2>Hack Club</h2>
            <div>
              <label className={styles.required}>
                How did you find out about this?
              </label>
              <textarea name="discovery"></textarea>
            </div>
            <div className={styles.flex}>
              <div>
                <label>Is there anything we're doing really well?</label>
                <textarea name="compliments"></textarea>
              </div>
              <div>
                <label>Likewise, anything we could improve on?</label>
                <textarea name="improvements"></textarea>
              </div>
            </div>
            <div>
              <button type="submit">Get my fudge!</button>
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
