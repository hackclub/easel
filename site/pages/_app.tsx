import '@/styles/globals.scss'
import type { AppProps } from 'next/app'
import Script from 'next/script'
import { Toaster } from 'react-hot-toast'

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Script
        defer
        data-domain="easel.hackclub.com"
        src="https://plausible.io/js/script.js"
      />
      {/* <Toaster /> */}
      <Component {...pageProps} />
    </>
  )
}
