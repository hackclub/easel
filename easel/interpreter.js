import Ast, { Return } from './ast.js'

Array.prototype.add = function (args) {
  console.log(this)
}

export class ReturnException extends Error {
  constructor(value) {
    super()
    this.value = value
  }
}

export class Interpreter {
  error(node, msg) {
    throw new Error(msg)
  }

  search(scope, key, msg) {}

  evaluate(value, scope) {
    switch (value.constructor) {
      case Ast.Var:
        if (!Object.keys(scope).includes(value.name))
          this.error(value, `${value.name} is not defined in current scope`)
        return scope[value.name]
      case Ast.Instance:
        if (!Object.keys(scope).includes(value.name))
          this.error(value, `${value.value} is not defined in current scope`)
        const constructor = scope[value.name]

        let members = {}
        for (let [member, memberValue] of Object.entries(value.members)) {
          members[member] = this.evaluate(memberValue, scope)
        }

        return constructor(members)
      case Ast.Call: {
        const caller = this.evaluate(value.caller, scope)
        if (!caller)
          this.error(value, `${value.name} is not defined in current scope`)
        let args = []
        for (let arg of value.args) args.push(this.evaluate(arg, scope))
        return caller(args)
      }
      case Ast.Get:
        const caller = this.evaluate(value.caller, scope)
        return caller[value.property.value]
      case Ast.Binary:
        const operations = {
          '+': (left, right) => left + right,
          '-': (left, right) => left - right,
          '*': (left, right) => left * right,
          '/': (left, right) => left / right,
          '||': (left, right) => left || right,
          '&&': (left, right) => left && right,
          '==': (left, right) => left == right,
          '!=': (left, right) => left != right,
          '>': (left, right) => left > right,
          '>=': (left, right) => left >= right,
          '<': (left, right) => left < right,
          '<=': (left, right) => left <= right
        }
        return operations[value.operator](
          this.evaluate(value.left, scope),
          this.evaluate(value.right, scope)
        )
      case Ast.Literal:
        return value.value
      default:
      // this.error(value, 'Unexpected')
    }
  }

  execute(node, scope) {
    switch (node.constructor) {
      case Ast.Var:
        scope[node.name] = this.evaluate(node.value, scope)
        break
      case Ast.Struct:
        scope[node.name] = options => {
          // Make sure there are no invalid keys
          let instance = {}
          for (let key of Object.keys(options)) {
            if (!node.members.includes(key))
              this.error(node, `Unexpected member ${key}`)
            instance[key] = options[key]
          }
          return instance
        }
        break
      case Ast.Func:
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
        break
      case Ast.Return:
        throw new ReturnException(this.evaluate(node.value, scope))
      case Ast.For:
        let localScope = { ...scope, [node.id]: this.evaluate(node.range[0]) }
        while (localScope[node.id] < this.evaluate(node.range[1])) {
          this.run(node.body, localScope)
          localScope[node.id]++
        }
        break
      case Ast.While:
        while (this.execute(node.condition, scope)) this.run(node.body, scope)
        break
      case Ast.Conditional:
        if (this.execute(node.condition, scope)) this.run(node.body, scope)
        else
          for (let conditional of node.otherwise)
            this.execute(conditional, scope)
        break
      default:
        return this.evaluate(node, scope)
    }
  }

  run(ast, scope) {
    for (let node of ast) {
      this.execute(node, scope)
    }
    return scope
  }
}
