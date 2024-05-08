/** @type {import('next').NextConfig} */
const nextConfig = {
  typescript: {
    ignoreBuildErrors: true
  },
  experimental: {
    serverActions: {
      bodySizeLimit: '50mb'
    }
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
