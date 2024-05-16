import '@/styles/globals.scss'
import type { AppProps } from 'next/app'
import Script from 'next/script'
import Meta from '@hackclub/meta'
import Head from 'next/head'

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Script
        defer
        data-domain="easel.hackclub.com"
        src="https://plausible.io/js/script.js"
      />
      <Meta
        as={Head}
        name="Orpheus' Hacky Guide to Writing a Programming Language"
        description="Learn how to write a programming language in a weekend!"
        image="https://cloud-bimy66myq-hack-club-bot.vercel.app/0video-capture-8135_2-min.png"
        color="#ec3750"
      />
      <Component {...pageProps} />
    </>
  )
}
