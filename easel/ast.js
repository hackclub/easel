export class Literal {
  constructor(value) {
    this.value = value
  }
}

export class Binary {
  constructor(left, operator, right) {
    this.left
    this.operator = operator
    this.right = right
  }
}

export class Var {
  constructor(name, value) {
    this.name = name
    this.value = value
  }
}

export class Struct {
  constructor(name, members) {
    this.name = name
    this.members = members
  }
}

export class Func {
  constructor(name, params, body) {
    this.name = name
    this.params = params
    this.body = body
  }
}

export class Call {
  constructor(caller, args) {
    this.caller = caller
    this.args = args
  }
}

export class Get {
  constructor(caller, property) {
    this.caller = caller
    this.property = property
  }
}

export class For {
  constructor(id, range, body) {
    this.id = id
    this.range = range
    this.body = body
  }
}

export class While {
  constructor(condition, body) {
    this.condition = condition
    this.body = body
  }
}

export default {
  Literal,
  Binary,
  Var,
  Struct,
  Func,
  Call,
  Get,
  For,
  While
}
