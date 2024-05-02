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
}

export default nextConfig
