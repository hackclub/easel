import fs from 'fs'
import { Interpreter  } from './interpreter.js'
import stdlib, { MeowError } from './stdlib.js'
import { Lexer } from './lexer.js'
import { Parser } from './parser.js'
import readline from 'node:readline'

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
    const debug = argv.find(cmd => cmd == '--dgb') ? true : false
    argv = argv.filter(arg => arg != '--dbg')

    const location = argv[0]
    if(location) {
        const program = await readFile(location)

        const lexer = new Lexer(program)
        try {
            lexer.scanTokens()
        } catch (err) {
            console.log(err)
            process.exit(1)
        } finally {
            if (debug) await writeFile('tokens.json', JSON.stringify(lexer.tokens, null, 2))
        }

        const parser = new Parser(lexer.tokens)
        try {
            parser.parse()
        } catch (err) {
            console.log(err)
        } finally {
            if (debug) await writeFile('ast.json', JSON.stringify(parser.ast,null,2))
        }

        const interpreter = new Interpreter()
        try {
            interpreter.run(parser.ast, stdlib)
        } catch (err) {
            console.log(err)
        }

    } else{
        const interpreter = new Interpreter()
        let scope = {
            ...stdlib,
            exit: () => process.exit(0)
        }

        const input = readline.createInterface({
            input: process.stdin,
            output: process.stdout
        })

        process.on('SIGINT', () => {
            input.close()
        })

        const repl = line => {
            let hadError = false

            const lexer = new Lexer(line)
            try {
                lexer.scanTokens()
            } catch (err) {
                if (err instanceof MeowError) {
                    hadError = true
                    console.log(err.toString())
                } else throw err
            }

            if (!hadError) {
                const parser = new Parser(lexer.tokens)
                try {
                    parser.parse()
                } catch(err) {
                    if (err instanceof MeowError) console.log(err.toString())
                    else throw err
                }

                try {
                    scope = interpreter.run(parser.ast, scope)
                } catch (err) {
                    if (err instanceof MeowError) console.log(err.toString())
                    else throw err
                }
            }
            input.question('> ', repl)
        }

        input.question('> ', repl)
    }
})()


  