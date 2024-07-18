import { MeowError } from "./stdlib.js";
import { TOKENS } from "./lexer.js";
import Ast from './ast.js'



const isOp = type =>
    [
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
        TOKENS.Mod,
        TOKENS.Asterisk,
        TOKENS.Slash,
    ].includes(type)

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
    '%': 1.5,
    '*': 2,
    '/': 2
}

export class Parser{
    constructor(tokens) {
        this.tokens = tokens
        this.ast = []
        this.current = 0
        this.catHappy = 100
    }

    error (token, msg) {
        throw new MeowError(
            'Syntax error on ' + (token.line) +':' + (token.column) + ':' + msg
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

    eat(type){
        if (this.peekType() == type) return this.tokens[this.current++]
        this.error(
            this.peek(),
            'Expected ' + type + ' but got ' + this.peekType().toString()
        )
    }
    
    exprList() {
        let exprs = []
        exprs.push(this.expr())
        while (this.peekType() == TOKENS.Comma) {
            this.eat(TOKENS.Comma)
            exprs.push(this.expr())
        }
        return exprs
    }

    peekKeyword(keyword) {
        if (this.peekType() != TOKENS.Keyword || this.peek().value != keyword)
            return null
        return this.peek()
    }

    eatKeyword(keyword) {
        if (this.peekType() != TOKENS.Keyword)
            this.error(
                this.peek(),
                'Expected ' + TOKENS.Keyword +' but got ' + this.peekType()
        )
        else if (this.peek().value != keyword)
            this.error(
                this.peek(),
                'Expected keyword ' + keyword + ' but got keyword ' + this.peek().value
        )
        return this.eat(TOKENS.Keyword)
    }

    identifierList() {
        let identifiers = []
        identifiers.push(this.eat(TOKENS.Identifier).value)
        while (this.peekType() == TOKENS.Comma) {
            this.eat(TOKENS.Comma)
            identifiers.push(this.eat(TOKENS.Identifier).value)
        }
        return identifiers
    }



    simple() {
        let token = this.eat(this.peekType())
        switch (token.type){
            case TOKENS.Keyword: {
                if (token.value == 'meeeoow') {
                    const id = this.eat(TOKENS.Identifier).value
                    
                    this.eat(TOKENS.LeftParen)
                    let members = {}
                    while (this.peekType() != TOKENS.RightParen) {
                        const member = this.eat(TOKENS.Identifier).value
                        this.eat(TOKENS.Colon)
                        members[member] = this.expr()
                        if (this.peekType() == TOKENS.Comma) this.eat(TOKENS.Comma)
                    }
                    this.eat(TOKENS.RightParen)

                    return new Ast.Instance(id, members)
                }
                break
            }
            case TOKENS.String:
            case TOKENS.Number:
            case TOKENS.Boolean: {
                return new Ast.Literal(token.content)
            }
            case TOKENS.LeftBracket: {
                let items = []
                if (this.peekType() != TOKENS.RightBracket) items = this.exprList()
                this.eat(TOKENS.RightBracket)
                return new Ast.Array(items)
            }
            case TOKENS.Identifier: {
                if (this.peekType() == TOKENS.Equal) {
                    this.eat(TOKENS.Equal)
                    let val = this.expr()
                    return new Ast.Var(token.value, val)
                }
                if (this.peekType() == TOKENS.PlusEquiv) {
                    this.eat(TOKENS.PlusEquiv)
                    let val = this.expr()
                    return new Ast.Changer(token.value, val, '+')
                }
                if (this.peekType() == TOKENS.MinusEquiv) {
                    this.eat(TOKENS.MinusEquiv)
                    let val = this.expr()
                    return new Ast.Changer(token.value, val, '-')
                }
                if (this.peekType() == TOKENS.PlusPlus) {
                    this.eat(TOKENS.PlusPlus)
                    let val = new Ast.Literal(1)
                    return new Ast.Changer(token.value, val, '+')
                }
                if (this.peekType() == TOKENS.MinusMinus) {
                    this.eat(TOKENS.MinusMinus)
                    let val = new Ast.Literal(1)
                    return new Ast.Changer(token.value, val, '-')
                }
                return new Ast.Var(token.value)
            }
            case TOKENS.LeftParen: {
                const expr = this.expr()
                this.eat(TOKENS.RightParen)
                return expr
            }
            case TOKENS.Feed: {
                this.eat(TOKENS.LeftParen)
                this.eat(TOKENS.RightParen)
                let amm = Math.random() * 15 + 3
                this.catHappy += amm
                return new Ast.Feeder("meow")
            }
            case TOKENS.Mow: {
                this.eat(TOKENS.LeftParen)
                this.eat(TOKENS.RightParen)
                let out = this.catHappy
                return new Ast.Literal(out)
            }
        }
        this.error(token, "Expected expression but got " + token)
    }

    call() {
        let expr = this.simple()
        while (true) {
            if (this.peekType() == TOKENS.LeftParen) {
                this.eat(TOKENS.LeftParen)
                let args = []
                if (this.peekType() != TOKENS.RightParen) args = this.exprList()
                this.eat(TOKENS.RightParen)
                expr = new Ast.Call(expr, args)
            } else if (this.peekType() == TOKENS.LeftBracket) {
                this.eat(TOKENS.LeftBracket)
                const property = this.expr()
                this.eat(TOKENS.RightBracket)
                expr = new Ast.Get(expr, property, true)
            } else if (this.peekType() == TOKENS.Period) {
                this.eat(TOKENS.Period) 
                const property = this.eat(TOKENS.Identifier).value
                expr = new Ast.Get(expr, property)
            } else break
        }
        return expr
    }

    unary() {
        if (this.peekType() == TOKENS.Not) {
            const op = this.eat(this.peekType()).value
            return new Ast.Unary(op, this.unary())
        }

        return this.call()
    }

    expr() {
        const left = this.unary()
        if (isOp(this.peekType())) {
            const op = this.eat(this.peekType()).value
            let right = this.expr()
            if (right instanceof Ast.Binary && opOrder[op] > opOrder[right.operator])
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
        const returnStmt = () => {
            this.eatKeyword('prrr')
            return new Ast.Return(this.expr())
        }

        const funcStmt = () => {
            this.eatKeyword('mmeow')
            const name = this.eat(TOKENS.Identifier).value

            let params = []
            if (this.peekKeyword('mmmeow')) {
                this.eatKeyword('mmmeow')
                this.eat(TOKENS.LeftParen)
                params = this.identifierList()
                this.eat(TOKENS.RightParen)
            }

            this.eat(TOKENS.LeftBrace)
            let body = []
            while (this.peekType() != TOKENS.RightBrace) body.push(this.stmt())
            this.eat(TOKENS.RightBrace)
            
            return new Ast.Func(name, params, body)
        }

        const forStmt = () => {
            this.eatKeyword('meeow')
            const id = this.eat(TOKENS.Identifier).value
            this.eatKeyword('meeeow')

            this.eat(TOKENS.LeftParen)
            const range = this.exprList()
            if (range.length != 2)
                this.error(
                    range[range.length - 1],
                    'Expected (start,end) range but received more arguments than expected'
            )
            this.eat(TOKENS.RightParen)

            this.eat(TOKENS.LeftBrace)
            let body = []
            while (this.peekType() != TOKENS.RightBrace) body.push(this.stmt())
            this.eat(TOKENS.RightBrace)

            return new Ast.For(id, range, body)
        }

        const whileStmt = () => {
            this.eatKeyword('mmeeooww')

            this.eat(TOKENS.LeftParen)
            const condition = this.expr()
            this.eat(TOKENS.RightParen)

            this.eat(TOKENS.LeftBrace)
            let body = []
            while (this.peekType() != TOKENS.RightBrace) body.push(this.stmt())
            this.eat(TOKENS.RightBrace)

            return new Ast.While(condition, body)
        }

        const conditionalStmt = keyword => {
            this.eatKeyword(keyword)

            let condition = new Ast.Literal(true)
            if (keyword != 'mroow') {
                this.eat(TOKENS.LeftParen)
                condition = this.expr()
                this.eat(TOKENS.RightParen)
            }

            this.eat(TOKENS.LeftBrace)
            let body = []
            while (this.peekType() != TOKENS.RightBrace) body.push(this.stmt())
            this.eat(TOKENS.RightBrace)

            let otherwise = []
            while (this.peekKeyword('mroooww') || this.peekKeyword('mroow'))
                otherwise.push(conditionalStmt(this.peek().value))

            return new Ast.Conditional(condition, body, otherwise)
        }

        const assignStmt = () => {
            this.eatKeyword('meow')
            const name = this.eat(TOKENS.Identifier).value
            if (this.peekType() == TOKENS.Period) {
                this.eat(TOKENS.Period)
                const property = this.eat(TOKENS.Identifier).value
                this.eatKeyword('meoow')
                const value = this.expr()
                return new Ast.Set(name, property, value)
            }
            this.eatKeyword('meoow')
            const value = this.expr()
            return new Ast.Var(name, value)
        }

        const structStmt = () => {
            this.eatKeyword('meeoow')
            const name = this.eat(TOKENS.Identifier).value
            this.eatKeyword('meeooow')
            this.eat(TOKENS.LeftBrace)
            const members = this.identifierList()
            this.eat(TOKENS.RightBrace)
            return new Ast.Struct(name, members)
        }
        const next = this.peek()
        switch (next.type) {
            case TOKENS.Keyword: {
                switch (next.value) {
                    case 'mmeow': {
                        return funcStmt()
                    }
                    case 'prrr': {
                        return returnStmt()
                    }
                    case 'meeow': {
                        return forStmt()
                    }
                    case 'mmeeooww': {
                        return whileStmt()
                    }
                    case 'mrow': {
                        return conditionalStmt('mrow')
                    }
                    case 'meow': {
                        return assignStmt()
                    }
                    case 'meeoow': {
                        return structStmt()
                    }
                }
            }
            default: {
                return this.expr()
            }
        }
    }
    parse() {
        while ((this.peekType() != TOKENS.EOF) && this.catHappy > 0){
            this.ast.push(this.stmt())
            this.catHappy -= (Math.random() *5)
        }
        if(this.catHappy < 0) {
            console.log("the cat ate all your code...")
        }
        return this.ast
    }
}