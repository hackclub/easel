import { EaselError } from './stdlib.js'
import { TOKENS } from './lexer.js'
import Ast from './ast.js'

const opOrder = {
  '<': 0,
  '<=': 0,
  '>': 0,
  '>=': 0,
  '!=': 0,
  '==': 0,
  '&&': 0,
  '||': 0,
  '+': 1,
  '-': 1,
  '*': 2,
  '/': 2
}

const isOp = type => {
  return [
    TOKENS.Or,
    TOKENS.And,
    TOKENS.Equiv,
    TOKENS.NotEquiv,
    TOKENS.Gt,
    TOKENS.Gte,
    TOKENS.Lt,
    TOKENS.Lte,
    TOKENS.Plus,
    TOKENS.Minus,
    TOKENS.Asterisk,
    TOKENS.Slash
  ].includes(type)
}

export class Parser {
  constructor(tokens) {
    this.tokens = tokens
    this.ast = []
    this.current = 0
  }

  error(token, msg) {
    throw new EaselError(
      `Syntax error on ${token.line}:${token.column}: ${msg}`
    )
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
    if (this.peekType() !== TOKENS.Keyword || this.peek().value !== keyword)
      return null
    return this.peek()
  }

  eat(type) {
    if (this.peekType() === type) return this.tokens[this.current++]
    this.error(this.peek(), `Expected ${type} but got ${this.peekType()}`)
  }

  eatKeyword(keyword) {
    if (this.peekType() !== TOKENS.Keyword)
      this.error(
        this.peek(),
        `Expected ${TOKENS.Keyword} but got ${this.peekType()}`
      )
    else if (this.peek().value !== keyword)
      this.error(
        this.peek(),
        `Expected keyword ${keyword} but got keyword ${this.peek().value}`
      )
    return this.eat(TOKENS.Keyword)
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
    identifiers.push(this.eat(TOKENS.Identifier).value)
    while (this.peekType() === TOKENS.Comma) {
      this.eat(TOKENS.Comma)
      identifiers.push(this.eat(TOKENS.Identifier).value)
    }
    return identifiers
  }

  simple() {
    let token = this.eat(this.peekType())
    switch (token.type) {
      case TOKENS.Identifier:
        return new Ast.Var(token.value)
      case TOKENS.Keyword:
        if (token.value === 'prep') {
          // Instance is essentially a struct copy
          const id = this.eat(TOKENS.Identifier).value

          this.eat(TOKENS.LeftParen)
          let members = {}
          while (this.peekType() !== TOKENS.RightParen) {
            const member = this.eat(TOKENS.Identifier).value
            this.eat(TOKENS.Colon)
            members[member] = this.expr()
            if (this.peekType() === TOKENS.Comma) this.eat(TOKENS.Comma)
          }
          this.eat(TOKENS.RightParen)

          return new Ast.Instance(id, members)
        }
      case TOKENS.String:
      case TOKENS.Number:
      case TOKENS.Boolean:
        return new Ast.Literal(token.content)
      case TOKENS.LeftBracket:
        let items = []
        if (this.peekType() !== TOKENS.RightBracket) items = this.exprList()
        this.eat(TOKENS.RightBracket)
        return new Ast.Array(items)
      case TOKENS.LeftParen:
        const expr = this.expr()
        this.eat(TOKENS.RightParen)
        return expr
    }
    this.error(token, 'Expected expression but got ' + token.type)
  }

  call() {
    let expr = this.simple()
    while (true) {
      if (this.peekType() === TOKENS.LeftParen) {
        this.eat(TOKENS.LeftParen)
        let args = []
        if (this.peekType() !== TOKENS.RightParen) args = this.exprList()
        this.eat(TOKENS.RightParen)
        expr = new Ast.Call(expr, args)
      } else if (this.peekType() === TOKENS.LeftBracket) {
        // This is also a getter, but we accept an expression rather than just an identifier
        this.eat(TOKENS.LeftBracket)
        const property = this.expr()
        this.eat(TOKENS.RightBracket)
        expr = new Ast.Get(expr, property, true)
      } else if (this.peekType() === TOKENS.Period) {
        this.eat(TOKENS.Period)
        const property = this.eat(TOKENS.Identifier).value
        expr = new Ast.Get(expr, property)
      } else break
    }
    return expr
  }

  unary() {
    // unary has one side
    if (this.peekType() === TOKENS.Not) {
      const op = this.eat(this.peekType()).value
      return new Ast.Unary(op, this.unary())
    }

    return this.call()
  }

  expr() {
    // expr has two sides
    let left = this.unary()
    if (isOp(this.peekType())) {
      const op = this.eat(this.peekType()).value
      let right = this.expr()
      if (right instanceof Ast.Binary && opOrder[op] > opOrder[right.operator])
        // Quick reordering based on precedence
        return new Ast.Binary(
          new Ast.Binary(left, op, right.left),
          right.operator,
          right.right
        )
      return new Ast.Binary(left, op, right)
    }
    return left
  }

  stmt() {
    const assignStmt = () => {
      this.eatKeyword('prepare')
      const name = this.eat(TOKENS.Identifier).value
      if (this.peekType() === TOKENS.Period) {
        // Setter
        this.eat(TOKENS.Period)
        const property = this.eat(TOKENS.Identifier).value
        this.eatKeyword('as')
        const value = this.expr()
        return new Ast.Set(name, property, value)
      }
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
        // Parameters
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

    const returnStmt = () => {
      this.eatKeyword('finished')
      return new Ast.Return(this.expr())
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

      return new Ast.While(condition, body)
    }

    const conditionalStmt = keyword => {
      this.eatKeyword(keyword)

      let condition = new Ast.Literal(true)
      if (keyword !== 'else') {
        this.eat(TOKENS.LeftParen)
        condition = this.expr()
        this.eat(TOKENS.RightParen)
      }

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
        switch (next.value) {
          case 'prepare':
            return assignStmt()
          case 'brush':
            return structStmt()
          case 'sketch':
            return funcStmt()
          case 'finished':
            return returnStmt()
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
