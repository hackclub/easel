import fs from 'fs'
import readline from 'node:readline'
import { Lexer } from './lexer.js'
import utils from './utils.js'

const readFile = location =>
  new Promise((resolve, reject) =>
    fs.readFile(location, 'utf-8', (err, data) => {
      if (err) return reject(err)
      resolve(data.toString())
    })
  )

const writeFile = (location, data) =>
  new Promise((resolve, reject) =>
    fs.writeFile(location, data, err => {
      if (err) return reject(err)
      resolve()
    })
  )

;(async () => {
  let argv = process.argv.slice(2)
  const debug = argv.find(cmd => cmd === '--dbg') ? true : false
  argv = argv.filter(arg => arg !== '--dbg')

  const location = argv[0]
  if (location) {
    const program = await readFile(location)
    const lexer = new Lexer(program)
    lexer.scanTokens()
    if (debug) await writeFile('tokens.json', JSON.stringify(lexer.tokens))
  } else {
    // Interactive REPL
    const input = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    })

    // Remember to close stream before exiting
    process.on('SIGINT', () => {
      input.close()
    })

    const repl = line => {
      const lexer = new Lexer(line)
      try {
        lexer.scanTokens()
      } catch {
        // Should catch errors, and depending on type, wait for extension
      }
      console.log(lexer.tokens)
      input.question('> ', repl)
    }

    input.question('> ', repl)
  }
})()
