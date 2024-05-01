/** @type {import('next').NextConfig} */
const nextConfig = {
  typescript: {
    ignoreBuildErrors: true
  },
  async redirects() {
    return [
      {
        source: '/',
        destination: '/orpheus-finds-easel',
        permanent: true
      }
    ]
  }
  // async headers() {
  //   return [
  //     {
  //       source: '/:slug',
  //       headers: [
  //         {
  //           key: 'Cross-Origin-Embedder-Policy',
  //           value: 'require-corp'
  //         },
  //         {
  //           key: 'Cross-Origin-Opener-Policy',
  //           value: 'same-origin'
  //         }
  //       ]
  //     }
  //   ]
  // }
}

export default nextConfig
