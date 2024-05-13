import fs from 'fs'
import readline from 'node:readline'
import { Lexer } from '../lexer.js'
import { Parser } from '../parser.js'
import { Interpreter } from '../interpreter.js'
import stdlib, { Canvas, EaselError } from '../stdlib.js'

class CustomCanvas extends Canvas {
  draw() {
    console.log(this.rows, this.cols)
    for (let row = 0; row < this.rows; row++) {
      for (let col = 0; col < this.cols; col++) {
        let cell = this.grid[row * this.cols + col]
        process.stdout.write(`\x1b[48;2;${cell.r};${cell.g};${cell.b}m `)
      }
      process.stdout.write('\n')
    }
  }
}

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
      if (err instanceof EaselError) console.log(err)
      process.exit(1)
    } finally {
      if (debug)
        await writeFile('tokens.json', JSON.stringify(lexer.tokens, null, 2))
    }

    const parser = new Parser(lexer.tokens)
    try {
      parser.parse()
    } catch (err) {
      if (err instanceof EaselError) console.log(err)
    } finally {
      if (debug)
        await writeFile('ast.json', JSON.stringify(parser.ast, null, 2))
    }

    const interpreter = new Interpreter()
    let scope = {
      ...stdlib,
      Canvas: new CustomCanvas()
    }
    try {
      scope = interpreter.run(parser.ast, scope)
      scope.Canvas.draw()

      if (interpreter.inScope(scope, 'painting')) {
        setInterval(() => {
          process.stdout.write('\x1Bc')
          const lexer = new Lexer('painting()')
          lexer.scanTokens()
          const parser = new Parser(lexer.tokens)
          parser.parse()
          scope = interpreter.run(parser.ast, scope)
          scope.Canvas.draw()
        }, 1000)
      }
    } catch (err) {
      if (err instanceof EaselError) console.log(err.toString())
    }
  } else {
    // Interactive REPL
    const interpreter = new Interpreter()
    let scope = {
      ...stdlib,
      exit: () => process.exit(0)
    }

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
        } else throw err
      }

      if (!hadError) {
        const parser = new Parser(lexer.tokens)
        try {
          parser.parse()
        } catch (err) {
          if (err instanceof EaselError) {
            // Check if start of block statement
          } else throw err
        }

        try {
          scope = interpreter.run(parser.ast, scope)
        } catch (err) {
          if (err instanceof EaselError) console.log(err.toString())
          else throw err
        }
      }

      input.question('> ', repl)
    }

    input.question('> ', repl)
  }
})()
