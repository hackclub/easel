class Set:
    def __init__(self, name, value, cursed=0):
        self.name = name
        self.value = value
        self.cursed = cursed

    __match_args__ = ("name", "value", "cursed")


class Var:
    def __init__(self, name, cursed=0):
        self.name = name
        self.cursed = cursed

    __match_args__ = ("name", "cursed")


class Function:
    def __init__(self, name, params, body, cursed=0):
        self.name = name
        self.params = params
        self.body = body
        self.cursed = cursed

    __match_args__ = ("name", "params", "body", "cursed")


class Return:
    def __init__(self, value, cursed=0):
        self.value = value
        self.cursed = cursed

    __match_args__ = ("value", "cursed")


class Conditional:
    def __init__(self, condition, body, else_body, cursed=0):
        self.condition = condition
        self.body = body
        self.else_body = else_body
        self.cursed = cursed

    __match_args__ = ("condition", "body", "else_body", "cursed")


class For:
    def __init__(self, variable, condition, increment, body, cursed=0):
        self.variable = variable
        self.condition = condition
        self.increment = increment
        self.body = body
        self.cursed = cursed

    __match_args__ = ("variable", "condition", "increment", "body", "cursed")


class While:
    def __init__(self, condition, body, cursed=0):
        self.condition = condition
        self.body = body
        self.cursed = cursed

    __match_args__ = ("condition", "body", "cursed")


class Call:
    def __init__(self, name, args, cursed=0):
        self.name = name
        self.args = args
        self.cursed = cursed

    __match_args__ = ("name", "args", "cursed")


class Get:
    def __init__(self, obj, prop, cursed=0):
        self.obj = obj
        self.prop = prop
        self.cursed = cursed

    __match_args__ = ("obj", "prop", "cursed")


class UnaryOp:
    def __init__(self, op, right, cursed=0):
        self.op = op
        self.right = right
        self.cursed = cursed

    __match_args__ = ("op", "right", "cursed")


class BinaryOp:
    def __init__(self, left, op, right, cursed=0):
        self.left = left
        self.op = op
        self.right = right
        self.cursed = cursed

    __match_args__ = ("left", "op", "right", "cursed")


class Literal:
    def __init__(self, value, cursed=0):
        self.value = value
        self.cursed = cursed

    __match_args__ = ("value", "cursed")


class CoughSyrup:
    pass
