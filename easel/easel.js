import fs from 'fs'
import readline from 'node:readline'
import { Lexer, TOKENS } from './lexer.js'
import { Parser } from './parser.js'
import { Interpreter } from './interpreter.js'
import stdlib, { EaselError } from './stdlib.js'

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
    try {
      lexer.scanTokens()
    } catch (err) {
      if (err instanceof EaselError) {
        console.log(err)
        process.exit(1)
      }
    } finally {
      if (debug) await writeFile('tokens.json', JSON.stringify(lexer.tokens))
    }

    const parser = new Parser(lexer.tokens)
    try {
      parser.parse()
    } catch (err) {
      if (err instanceof EaselError) {
        console.log(err)
        process.exit(2)
      }
    } finally {
      if (debug) await writeFile('ast.json', JSON.stringify(parser.ast))
    }

    const interpreter = new Interpreter()
    try {
      interpreter.run(parser.ast, stdlib)
    } catch (err) {
      if (err instanceof EaselError) console.log(err.toString())
    }
  } else {
    // Interactive REPL
    const interpreter = new Interpreter()
    let scope = stdlib

    const input = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    })

    // Remember to close stream before exiting
    process.on('SIGINT', () => {
      input.close()
    })

    const repl = line => {
      let hadError = false

      const lexer = new Lexer(line)
      try {
        lexer.scanTokens()
      } catch (err) {
        if (err instanceof EaselError) {
          hadError = true
          console.log(err.toString())
        }
      }

      if (!hadError) {
        const parser = new Parser(lexer.tokens)
        try {
          parser.parse()
        } catch (err) {
          if (err instanceof EaselError) {
            hadError = true
            console.log(err.toString())
          }
        }

        if (!hadError) {
          try {
            scope = interpreter.run(parser.ast, scope)
          } catch (err) {
            if (err instanceof EaselError) console.log(err.toString())
          }
        }
      }

      input.question('> ', repl)
    }

    input.question('> ', repl)
  }
})()
