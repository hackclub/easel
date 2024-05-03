const { lezer } = require('@lezer/generator/rollup')

module.exports = {
  input: './components/easel.grammar',
  output: [
    {
      format: 'es',
      file: './components/easel.js'
    }
  ],
  external: ['@lezer/lr', '@lezer/highlight'],
  plugins: [lezer()]
}
