import '@/styles/globals.scss'
import type { AppProps } from 'next/app'
import Script from 'next/script'

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Script
        defer
        data-domain="easel.hackclub.com"
        src="https://plausible.io/js/plausible.js"
      />
      <Component {...pageProps} />
    </>
  )
}
