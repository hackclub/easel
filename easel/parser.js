import { TOKENS, Token } from './lexer.js'
import Ast from './ast.js'

const isOp = token => {
  return [
    TOKENS.Or,
    TOKENS.Not,
    TOKENS.And,
    TOKENS.Equiv,
    TOKENS.Gt,
    TOKENS.Gte,
    TOKENS.Lt,
    TOKENS.Lte,
    TOKENS.Plus,
    TOKENS.Minus,
    TOKENS.Asterisk,
    TOKENS.Slash
  ].includes(token.type)
}

export class Parser {
  constructor(tokens) {
    this.tokens = tokens
    this.ast = []
    this.current = 0
  }

  error(token, msg) {
    throw new Error(`${token.line}:${token.column}: ${msg}`)
  }

  peek() {
    if (this.current >= this.tokens.length) return null
    return this.tokens[this.current]
  }

  peekType() {
    if (this.current >= this.tokens.length) return null
    return this.tokens[this.current].type
  }

  peekKeyword(keyword) {
    if (this.peekType() !== TOKENS.Identifier || this.peek().value !== keyword)
      return null
    return this.peek()
  }

  eat(type) {
    if (this.peekType() === type) return this.tokens[this.current++]
    this.error(this.peek(), `Expected ${type} but got ${this.peekType()}`)
  }

  eatKeyword(keyword) {
    if (this.peekType() !== TOKENS.Identifier)
      this.error(
        this.peek(),
        `Expected ${TOKENS.Identifier} but got ${this.peekType()}`
      )
    else if (this.peek().value !== keyword)
      this.error(
        this.peek(),
        `Expected identifier ${keyword} but got identifier ${this.peek().value}`
      )
    return this.eat(TOKENS.Identifier)
  }

  exprList() {
    let exprs = []
    exprs.push(this.expr())
    while (this.peekType() === TOKENS.Comma) {
      this.eat(TOKENS.Comma)
      exprs.push(this.expr())
    }
    return exprs
  }

  identifierList() {
    let identifiers = []
    identifiers.push(this.eat(TOKENS.Identifier))
    while (this.peekType() === TOKENS.Comma) {
      this.eat(TOKENS.Comma)
      identifiers.push(this.eat(TOKENS.Identifier))
    }
    return identifiers
  }

  simple() {
    let token = this.eat(this.peekType())
    switch (token.type) {
      case TOKENS.Identifier:
        return new Ast.Var(token.value)
      case TOKENS.String:
      case TOKENS.Number:
      case TOKENS.Boolean:
        return new Ast.Literal(token.content)
      case TOKENS.LeftBracket:
        let items = []
        if (this.peekType() !== TOKENS.RightBracket) items = this.exprList()
        this.eat(TOKENS.RightBracket)
        return new Ast.Literal(items)
    }
    this.error(token, 'Expected expression but got ' + token)
  }

  call() {
    let expr = this.simple()
    while (true) {
      if (this.peekType() == TOKENS.LeftParen) {
        this.eat(TOKENS.LeftParen)
        const args = this.exprList()
        this.eat(TOKENS.RightParen)
        expr = new Ast.Call(expr, args)
      } else if (this.peekType() == TOKENS.RightParen) {
        // This is also a getter, but we can accept an expression, rather than just an identifier
        this.eat(TOKENS.LeftBracket)
        const property = this.expr()
        this.eat(TOKENS.RightBracket)
        expr = new Ast.Get(expr, property)
      } else if (this.peekType() == TOKENS.Period) {
        this.eat(TOKENS.Period)
        const property = this.eat(TOKENS.Identifier)
        expr = new Ast.Get(expr, property)
      } else break
    }
    return expr
  }

  expr() {
    // expr has two sides
    const left = this.call()
    if (isOp(this.peekType())) {
      const op = this.eat(this.peekType()).value
      const right = this.expr()
      return new Ast.Binary(left, op, right)
    }
    return left
  }

  stmt() {
    const varStmt = () => {
      this.eatKeyword('prepare')
      const name = this.eat(TOKENS.Identifier).value
      this.eatKeyword('as')
      const value = this.expr()
      return new Ast.Var(name, value)
    }

    const structStmt = () => {
      this.eatKeyword('brush')
      const name = this.eat(TOKENS.Identifier).value
      this.eatKeyword('has')
      this.eat(TOKENS.LeftBrace)
      const members = this.identifierList()
      this.eat(TOKENS.RightBrace)
      return new Ast.Struct(name, members)
    }

    const funcStmt = () => {
      this.eatKeyword('sketch')
      const name = this.eat(TOKENS.Identifier).value

      let params = []
      if (this.peekKeyword('needs')) {
        // Parameters to pass in
        this.eatKeyword('needs')
        this.eat(TOKENS.LeftParen)
        params = this.identifierList()
        this.eat(TOKENS.RightParen)
      }

      this.eat(TOKENS.LeftBrace)
      let body = []
      while (this.peekType() !== TOKENS.RightBrace) body.push(this.stmt())
      this.eat(TOKENS.RightBrace)

      return new Ast.Func(name, params, body)
    }

    const forStmt = () => {
      this.eatKeyword('loop')
      const id = this.eat(TOKENS.Identifier).value
      this.eatKeyword('through')

      this.eat(TOKENS.LeftParen)
      const range = this.exprList()
      if (range.length !== 2)
        this.error(
          range[range.length - 1],
          'Expected (start, end) range but received more arguments than expected'
        )
      this.eat(TOKENS.RightParen)

      this.eat(TOKENS.LeftBrace)
      let body = []
      while (this.peekType() !== TOKENS.RightBrace) body.push(this.stmt())
      this.eat(TOKENS.RightBrace)

      return new Ast.For(id, range, body)
    }

    const whileStmt = () => {
      this.eatKeyword('while')

      this.eat(TOKENS.LeftParen)
      const condition = this.expr()
      this.eat(TOKENS.RightParen)

      this.eat(TOKENS.LeftBrace)
      let body = []
      while (this.peekType() !== TOKENS.RightBrace) body.push(this.stmt())
      this.eat(TOKENS.RightBrace)

      return Ast.While(condition, body)
    }

    const conditionalStmt = keyword => {
      this.eatKeyword(keyword)

      this.eat(TOKENS.LeftParen)
      const condition = this.expr()
      this.eat(TOKENS.RightParen)

      this.eat(TOKENS.LeftBrace)
      let body = []
      while (this.peekType() !== TOKENS.RightBrace) body.push(this.stmt())
      this.eat(TOKENS.RightBrace)

      let otherwise = []
      while (this.peekKeyword('elif') || this.peekKeyword('else'))
        otherwise.push(conditionalStmt(this.peek().value))

      return new Ast.Conditional(condition, body, otherwise)
    }

    const next = this.peek()
    switch (next.type) {
      case TOKENS.Keyword:
        // Keywords make statements
        switch (next.value) {
          case 'prepare':
            return varStmt()
          case 'brush':
            return structStmt()
          case 'sketch':
            return funcStmt()
          case 'loop':
            return forStmt()
          case 'while':
            return whileStmt()
          case 'if':
            return conditionalStmt('if')
        }
      default:
        return this.expr()
    }
  }

  parse() {
    while (this.peekType() !== TOKENS.EOF) this.ast.push(this.stmt())
    return this.ast
  }
}
