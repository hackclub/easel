import Ast from './ast.js'
import {MeowError} from './stdlib.js'

Array.prototype.add = function (args) {
    this.push(...args)
}

Array.prototype.get = function ([index]) {
    return this[index]
}

export class ReturnException extends Error {
    constructor(value) {
        super()
        this.value = value
    }
}

export class Interpreter {
    error(msg) {
        throw new MeowError('Runtime error: ' + msg)
    }

    run(ast, scope) {
        for (const node of ast) {
            scope = this.execute(node, scope)
        }
        return scope
    }

    inScope(scope, name) {
        return Object.keys(scope).includes(name)
    }

    evaluate(value, scope) {
        switch (value.constructor) {
            case Ast.Var: {
                if (!this.inScope(scope, value.name))
                    this.error(value.name + ' is not defined in current scope')
                return scope[value.name]
            }
            case Ast.Instance: {
                if (!this.inScope(scope, value.name))
                    this.error(value.name + ' is not defined in current scope')

                const constructor = scope[value.name]
                let members = {}
                for (let [member, memberValue] of Object.entries(value.members))
                    members[member] = this.evaluate(memberValue, scope)
                return constructor(members)
            }
            case Ast.Call: {
                const caller = this.evaluate(value.caller, scope)
                if (!caller) this.error('Caller did not resolve to a defined value')
                let args = []
                for (let arg of value.args) args.push(this.evaluate(arg, scope))
                return caller(args)
            }
            case Ast.Get:
                const caller = this.evaluate(value.caller, scope)

                let get
                if (value.isExpr) get = caller[this.evaluate(value.property, scope)]
                else get = caller[value.property]

                if (get instanceof Function) return get.bind(caller)
                return get
            case Ast.Unary: {
                const operations = {'!': apply => !apply}
                return operations[value.operator](this.evaluate(value.apply, scope))
            }
            case Ast.Binary: {
                const operations = {
                    '<': (left,right) => left < right,
                    '<=': (left,right) => left <= right,
                    '>': (left,right) => left > right,
                    '>=': (left,right) => left >= right,
                    '!=': (left,right) => left != right,
                    '==': (left,right) => left == right,
                    '&&': (left,right) => left && right,
                    '||': (left,right) => left || right,
                    '+': (left,right) => left + right,
                    '-': (left,right) => left - right,
                    '%': (left,right) => left % right,
                    '*': (left,right) => left * right,
                    '/': (left,right) => left / right
                }
                return operations[value.operator](
                    this.evaluate(value.left, scope),
                    this.evaluate(value.right, scope)
                )
            }
            case Ast.Literal: {
                return value.value
            }
            case Ast.Array: {
                return value.value.map(expr => this.evaluate(expr, scope))
            }
        }
    }

    execute(node, scope) {
        switch(node.constructor) {
            case Ast.Var: {
                let newVal = node.value
                scope[node.name] = this.evaluate(newVal, scope)
                return scope
            }
            case Ast.Set: {
                if (!this.inScope(scope, node.caller))
                    this.error(node.caller + ' is not defined in current scope')
                scope[node.caller][node.property] = this.evaluate(node.value, scope)
                return scope
            }
            case Ast.Changer: {
                if (node.opertor = '+') {
                    let newVal = scope[node.name] + this.evaluate(node.right, scope)
                    scope[node.name] = newVal
                    return scope 
                }
                if (node.opertor = '-') {
                    let newVal = scope[node.name] - this.evaluate(node.right, scope)
                    scope[node.name] = newVal
                    return scope 
                }
            }
            case Ast.Struct: {
                scope[node.name] = members => {
                    let instance = {}
                    for (let key of Object.keys(members)) {
                        if (!node.members.includes(key))
                            this.error('Unexpected member ' +key + ' found while creating isntance of ' + node.name)
                        instance[key] = members[key]
                    }
                    return instance
                }
                return scope
            }
            case Ast.Func: {
                const func = args => {
                    let localScope = { ...scope }
                    for (let [i, param] of node.params.entries())
                        localScope[param] = args[i]
                    try {
                        this.run(node.body, localScope)
                    } catch (err) {
                        if (err instanceof ReturnException) return err.value
                        else throw err
                    }
                }
                scope[node.name] = func
                return scope
            }
            case Ast.Return:
                throw new ReturnException(this.evaluate(node.value, scope))
            case Ast.While:{
                while (this.evaluate(node.condition, scope)) this.run(node.body, scope)
                break
            }
            case Ast.For: { 
                let localScope = { ...scope, [node.id]: this.evaluate(node.range[0], scope) }
                while (localScope[node.id] < this.evaluate(node.range[1], scope)){
                    this.run(node.body, localScope)
                    localScope[node.id]++
                }
                return localScope
            }
            case Ast.Conditional: {
                if (this.evaluate(node.condition, scope)) this.run(node.body, scope)
                else
                    for (const conditional of node.otherwise)
                        this.execute(conditional, scope)
                break
            }
            default:
                this.evaluate(node, scope)
        }
        return scope
    }
}