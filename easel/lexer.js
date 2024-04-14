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
  Or: 'Or',
  Not: 'Not',
  And: 'And',
  Equal: 'Equal',
  Equiv: 'Equiv',
  Gt: 'Gt',
  Gte: 'Gte',
  Lt: 'Lt',
  Lte: 'Lte',
  Plus: '+',
  Minus: '-',
  Asterisk: '*',
  Slash: '/',
  EOF: 'EOF'
}

export const KEYWORDS = {
  prepare: 'prepare',
  as: 'as', // Variables
  brush: 'brush',
  has: 'has', // Structs
  sketch: 'sketch',
  paint: 'paint',
  needs: 'needs',
  finished: 'finished', // Functions
  loop: 'loop',
  through: 'through',
  while: 'while', // Loops
  if: 'if',
  elif: 'elif',
  else: 'else' // Conditionals
}

export class Token {
  constructor(type, value, content) {
    this.type = type
    this.value = value
    this.content = content
  }
}

export class Lexer {
  constructor(program) {
    this.program = program
    this.tokens = []
    this.current = 0
    this.line = 1
    this.col = 0
  }

  error(msg) {
    throw new Error(`${this.line}:${this.col}: ${msg}`)
  }

  peek() {
    if (this.current >= this.program.length) return '\0'
    return this.program[this.current]
  }

  advance() {
    if (this.current >= this.program.length) return '\0'
    this.col++
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
      char == '_'

    switch (char) {
      case '>':
        if (this.match('='))
          return this.tokens.push(new Token(TOKENS.Gte, '>=', '>='))
        return this.tokens.push(new Token(TOKENS.Gt, '>', '>'))
      case '<':
        if (this.match('='))
          return this.tokens.push(new Token(TOKENS.Lte, '<=', '<='))
        return this.tokens.push(new Token(TOKENS.Lt, '<', '<'))
      case '=':
        if (this.match('='))
          return this.tokens.push(new Token(TOKENS.Equiv, '==', '=='))
        return this.tokens.push(new Token(TOKENS.Equal, '=', '='))
      case '(':
        return this.tokens.push(new Token(TOKENS.LeftParen, '(', '('))
      case ')':
        return this.tokens.push(new Token(TOKENS.RightParen, ')', ')'))
      case '{':
        return this.tokens.push(new Token(TOKENS.LeftBrace, '{', '{'))
      case '}':
        return this.tokens.push(new Token(TOKENS.RightBrace, '}', '}'))
      case '[':
        return this.tokens.push(new Token(TOKENS.LeftBracket, '[', '['))
      case ']':
        return this.tokens.push(new Token(TOKENS.RightBracket, ']', ']'))
      case '|':
        if (this.match('|'))
          return this.tokens.push(new Token(TOKENS.Or, '||', '||'))
      case '&':
        if (this.match('&'))
          return this.tokens.push(new Token(TOKENS.And, '&&', '&&'))
      case '!':
        return this.tokens.push(new Token(TOKENS.Not, '!', '!'))
      case '.':
        return this.tokens.push(new Token(TOKENS.Period, '.', '.'))
      case ',':
        return this.tokens.push(new Token(TOKENS.Comma, ',', ','))
      case ':':
        return this.tokens.push(new Token(TOKENS.Colon, ':', ':'))
      case '+':
        return this.tokens.push(new Token(TOKENS.Plus, '+', '+'))
      case '-':
        return this.tokens.push(new Token(TOKENS.Minus, '-', '-'))
      case '*':
        return this.tokens.push(new Token(TOKENS.Asterisk, '*', '*'))
      case '/':
        return this.tokens.push(new Token(TOKENS.Slash, '/', '/'))
      case '~':
        // Comments
        while (this.peek() !== '\n') this.advance()
        return
      case ' ':
      case '\t':
        // Ignore whitespace
        return
      case '\n':
        // Also ignore, but update line
        this.line++
        this.col = 0
        return
      case "'":
      case '"':
        // String
        let string = []
        while (this.peek() !== char) {
          string.push(this.advance())
          if (this.peek() === '\0')
            // String wasn't closed
            this.error('Unexpected end of file; expected a closing quote')
        }
        this.advance() // Skip closing quote
        string = string.join('')
        return this.tokens.push(new Token(TOKENS.String, string, string))
      default:
        if (isNumber(char)) {
          let number = [char]
          while (
            isNumber(this.peek()) ||
            (this.peek() === '.' && !number.includes('.'))
          ) {
            number.push(this.advance())
          }
          number = number.join('')
          return this.tokens.push(
            new Token(TOKENS.Number, number, Number(number))
          )
        } else if (isChar(char)) {
          // Identifier or keyword
          let identifier = [char]
          while (isChar(this.peek())) identifier.push(this.advance())
          identifier = identifier.join('')
          if (Object.keys(KEYWORDS).includes(identifier))
            return this.tokens.push(
              new Token(TOKENS.Keyword, identifier, KEYWORDS[identifier])
            )
          return this.tokens.push(
            new Token(TOKENS.Identifier, identifier, identifier)
          )
        } else this.error('Unexpected symbol ' + char)
    }
  }

  scanTokens() {
    while (this.peek() != '\0') this.scanToken()
    return this.tokens
  }
}
