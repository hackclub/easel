import { MeowError } from './stdlib.js'

export const TOKENS = {
  LeftParen: 'LeftParen',
  RightParen: 'RightParen',
  LeftBrace: 'LeftBrace',
  RightBrace: 'RightBrace',
  LeftBracket: 'LeftBracket',
  RightBracket: 'RightBracket',
  Period: 'Period',
  Comma: 'Comma',
  Colon: 'Colon',
  Keyword: 'Keyword',
  Identifier: 'Identifier',
  String: 'String',
  Number: 'Number',
  Boolean: 'Boolean',
  Or: 'Or',
  Not: 'Not',
  Equal: 'Equal',
  Equiv: 'Equiv',
  NotEquiv: 'NotEquiv',
  Gt: 'Gt',
  Gte: 'Gte',
  Lt: 'Lt',
  Lte: 'Lte',
  Plus: 'Plus',
  PlusPlus: 'PlusPlus',
  PlusEquiv: 'PlusEquiv',
  MinusMinus: 'MinusMinus',
  MinusEquiv: 'MinusEquiv',
  Minus: 'Minus',
  Asterisk: 'Asterisk',
  Slash: 'Slash',
  Mod: 'Mod',
  Feed: 'Feed',
  Mow: 'Mow',
  EOF: 'EOF'
}

export const KEYWORDS = {
  meow: 'meow',
  meoow: 'meoow', // Variables
  meeoow: 'meeoow',
  meeeoow: 'meeeoow',
  meeooow: 'meeooow', // Structs
  mmeow: 'mmeow',
  mmmeow: 'mmmeow',
  prrr: 'prrr', // Functions
  meeow: 'meeow',
  meeeow: 'meeeow',
  mmeeooww: 'mmeeooww', // Loops
  mrow: 'mrow',
  mroow: 'mroow',
  mroooww: 'mroooww', // Conditionals
}

export class Token {
  constructor(type, value, content, line, column) {
    this.type = type
    this.value = value
    this.content = content
    this.line = line
    this.column = column
  }

  toString() {
    return this.value
  }
}

export class Lexer {
  constructor(program) {
    this.program = program
    this.tokens = []
    this.current = 0
    this.line = 1
    this.column = 1
  }

  error(msg) {
    throw new MeowError(`Error on ${this.line}:${this.column}: ${msg}`)
  }

  peek() {
    if (this.current >= this.program.length) return '\0'
    return this.program[this.current]
  }

  advance() {
    if (this.current >= this.program.length) return '\0'
    this.column++
    return this.program[this.current++]
  }

  match(char) {
    if (this.peek() === char) return this.advance()
    return false
  }

  scanToken() {
    const char = this.advance()

    const isNumber = char => char >= '0' && char <= '9'
    const isChar = char =>
      (char >= 'A' && char <= 'Z') ||
      (char >= 'a' && char <= 'z') ||
      char === '_'
    const isAlphanumeric = char => isNumber(char) || isChar(char)

    switch (char) {
      case '>': {
        if (this.match('='))
          return this.tokens.push(
            new Token(TOKENS.Gte, '>=', '>=', this.line, this.column)
          )
        return this.tokens.push(
          new Token(TOKENS.Gt, '>', '>', this.line, this.column)
        )
      }
      case '<': {
        if (this.match('='))
          return this.tokens.push(
            new Token(TOKENS.Lte, '<=', '<=', this.line, this.column)
          )
        return this.tokens.push(
          new Token(TOKENS.Lt, '<', '<', this.line, this.column)
        )
      }
      case '=': {
        if (this.match('='))
          return this.tokens.push(
            new Token(TOKENS.Equiv, '==', '==', this.line, this.column)
          )
        return this.tokens.push(new Token(TOKENS.Equal, '=', '=', this.line, this.column))
      }
      case '(': {
        return this.tokens.push(
          new Token(TOKENS.LeftParen, '(', '(', this.line, this.column)
        )
      }
      case ')': {
        return this.tokens.push(
          new Token(TOKENS.RightParen, ')', ')', this.line, this.column)
        )
      }
      
      case '[': {
        return this.tokens.push(
          new Token(TOKENS.LeftBracket, '[', '[', this.line, this.column)
        )
      }
      case ']': {
        return this.tokens.push(
          new Token(TOKENS.RightBracket, ']', ']', this.line, this.column)
        )
      }
      case '|': {
        if (this.match('|'))
          return this.tokens.push(
            new Token(TOKENS.Or, '||', '||', this.line, this.column)
          )
      }
      case '&': {
        if (this.match('&'))
          return this.tokens.push(
            new Token(TOKENS.And, '&&', '&&', this.line, this.column)
          )
      }
      case '!': {
        if (this.match('='))
          return this.tokens.push(
            new Token(TOKENS.NotEquiv, '!=', '!=', this.line, this.column)
          )
        return this.tokens.push(
          new Token(TOKENS.Not, '!', '!', this.line, this.column)
        )
      }
      case '.': {
        return this.tokens.push(
          new Token(TOKENS.Period, '.', '.', this.line, this.column)
        )
      }
      case ',': {
        return this.tokens.push(
          new Token(TOKENS.Comma, ',', ',', this.line, this.column)
        )
      }
      case ':': {
        return this.tokens.push(
          new Token(TOKENS.Colon, ':', ':', this.line, this.column)
        )
      }
      case '+': {
        if(this.match('+'))
          return this.tokens.push(
            new Token(TOKENS.PlusPlus, '++', '++', this.line, this.column)
        )
        if(this.match('='))
          return this.tokens.push(
            new Token(TOKENS.PlusEquiv, '+=', '+=', this.line, this.column)
        )
        return this.tokens.push(
          new Token(TOKENS.Plus, '+', '+', this.line, this.column)
        )
      }
      case '-': {
        if(this.match('-'))
          return this.tokens.push(
            new Token(TOKENS.MinusMinus, '--', '--', this.line, this.column)
        )
        if(this.match('='))
          return this.tokens.push(
            new Token(TOKENS.MinusEquiv, '-=', '-=', this.line, this.column)
        )
        return this.tokens.push(
          new Token(TOKENS.Minus, '-', '-', this.line, this.column)
        )
      }
      case '*': {
        return this.tokens.push(
          new Token(TOKENS.Asterisk, '*', '*', this.line, this.column)
        )
      }
      case '/': {
        return this.tokens.push(
          new Token(TOKENS.Slash, '/', '/', this.line, this.column)
        )
      }
      case '%': {
        return this.tokens.push(
          new Token(TOKENS.Mod, '%', '%', this.line, this.column)
        )
      }
      case '~': {
        // Comments
        while (this.peek() !== '\n' && this.peek() !== null) this.advance()
        return
      }
      case ' ':
      case '\r': {
        // Ignore whitespace
        return
      }
      case '\n': {
        // Also ignore, but update line
        this.line++
        this.column = 0
        return
      }
      case "'":
      case '"': {
        // String
        let string = []
        while (this.peek() !== char) {
          string.push(this.advance())
          if (this.peek() === null)
            // String wasn't closed
            this.error('Unexpected end of file; expected a closing quote')
        }
        this.advance() // Skip closing quote
        string = string.join('')
        return this.tokens.push(
          new Token(TOKENS.String, string, string, this.line, this.column)
        )
      }
      default: {
        if (isNumber(char)) {
          let number = [char]
          while (
            isNumber(this.peek()) ||
            (this.peek() === '.' && !number.includes('.'))
          )
            number.push(this.advance())
          number = number.join('')
          return this.tokens.push(
            new Token(
              TOKENS.Number,
              number,
              Number(number),
              this.line,
              this.column
            )
          )
        } else if (isChar(char)) {
          // Identifier or keyword
          let identifier = [char]
          while (isAlphanumeric(this.peek())) identifier.push(this.advance())
          identifier = identifier.join('')
          if (identifier == 'Meow') {
            return this.tokens.push(
              new Token(TOKENS.LeftBrace, "Meow", "Meow", this.line, this.column)
            )
          }
          if (identifier == 'meoW') {
            return this.tokens.push(
              new Token(TOKENS.RightBrace, "meoW", "meoW", this.line, this.column)
            )
          }
          if (identifier == 'feed') {
            return this.tokens.push(
              new Token(TOKENS.Feed, "feed", "feed", this.line, this.column)
            )
          }
          if (identifier == 'mow') {
            return this.tokens.push(
              new Token(TOKENS.Mow, "mow", "mow", this.line, this.column)
            )
          }
          if (Object.keys(KEYWORDS).includes(identifier))
            return this.tokens.push(
              new Token(
                TOKENS.Keyword,
                identifier,
                KEYWORDS[identifier],
                this.line,
                this.column
              )
            )
          else if (identifier === 'true' || identifier === 'false')
            return this.tokens.push(
              new Token(TOKENS.Boolean, identifier, identifier === 'true')
            )
          return this.tokens.push(
            new Token(
              TOKENS.Identifier,
              identifier,
              identifier,
              this.line,
              this.column
            )
          )
        } else this.error('Unexpected symbol ' + char)
      }
    }
  }

  scanTokens() {
    while (this.peek() != '\0') this.scanToken()
    this.tokens.push(new Token(TOKENS.EOF, '\0', '\0', this.line, this.column))
    return this.tokens
  }
}