from alexer import TOKEN_TYPE
from aast import (
    new_var,
    new_func,
    new_class,
    new_dict,
    new_array,
    new_return,
    new_if,
    new_else,
    new_while,
    new_for,
    new_number,
    new_string,
    new_bool,
    new_binop,
    new_call,
    new_attr,
    new_chain,
)
from sys import exit


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.current = 0

    def peek_token(self):
        if self.current >= len(self.tokens):
            return None
        return self.tokens[self.current]

    def peek_token_type(self):
        if self.current >= len(self.tokens):
            return None
        return self.tokens[self.current].get("type")

    def eat(self, type):
        if self.peek_token_type() == type:
            res = self.tokens[self.current]
            self.current += 1
            return res
        token = self.peek_token()
        raise Exception(f"Expected {type} but got {token.get('TYPE')}")


def stmt(parser):
    curr = parser.peek_token_type()
    if curr == TOKEN_TYPE["Var"]:
        return var_stmt(parser)
    elif curr == TOKEN_TYPE["Call"]:
        return call(parser)
    elif curr == TOKEN_TYPE["Func"]:
        return func_stmt(parser)
    elif curr == TOKEN_TYPE["Class"]:
        return class_stmt(parser)
    elif curr == TOKEN_TYPE["Return"]:
        return return_stmt(parser)
    elif curr == TOKEN_TYPE["For"]:
        return for_stmt(parser)
    elif curr == TOKEN_TYPE["While"]:
        return while_stmt(parser)
    elif curr == TOKEN_TYPE["If"]:
        return if_stmt(parser)
    else:
        return expr(parser)


def simple(parser):
    token = parser.eat(parser.peek_token_type())
    kind = token["type"]
    if kind == TOKEN_TYPE["Word"]:
        return new_var(token["value"])
    elif kind == TOKEN_TYPE["Minus"]:
        # Negative number
        return new_number(-simple(parser)["value"])
    elif kind == TOKEN_TYPE["Number"]:
        return new_number(token["value"])
    elif kind == TOKEN_TYPE["String"]:
        return new_string(token["value"])
    elif kind == TOKEN_TYPE["True"]:
        return new_bool(True)
    elif kind == TOKEN_TYPE["False"]:
        return new_bool(False)
    elif kind == TOKEN_TYPE["New"]:
        # New instance
        id = parser.eat(TOKEN_TYPE["Word"])
    elif kind == TOKEN_TYPE["LeftParen"]:
        # Left parentheses
        left = expr(parser, True)
        parser.eat(TOKEN_TYPE["RightParen"])
        return left
    elif kind == TOKEN_TYPE["LeftBracket"]:
        items = []
        if parser.peek_token_type() != TOKEN_TYPE["RightBracket"]:
            items.append(expr(parser))
            while parser.peek_token_type() == TOKEN_TYPE["Comma"]:
                parser.eat(TOKEN_TYPE["Comma"])
                items.append(expr(parser))
        parser.eat(TOKEN_TYPE["RightBracket"])
        return new_array(items)
    elif kind == TOKEN_TYPE["LeftBrace"]:
        obj = {}
        while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
            key = parser.eat(TOKEN_TYPE["String"])
            parser.eat(TOKEN_TYPE["Colon"])
            obj[key["value"]] = expr(parser)
            if parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
                parser.eat(TOKEN_TYPE["Comma"])
        parser.eat(TOKEN_TYPE["RightBrace"])
        return new_dict(obj)
    else:
        raise Exception("Expected expression but got " + kind)


def is_op(token):
    return token["type"] in [
        TOKEN_TYPE["Plus"],
        TOKEN_TYPE["Minus"],
        TOKEN_TYPE["Times"],
        TOKEN_TYPE["Divide"],
        TOKEN_TYPE["Modulo"],
        TOKEN_TYPE["LessThan"],
        TOKEN_TYPE["LessThanOrEqual"],
        TOKEN_TYPE["GreaterThan"],
        TOKEN_TYPE["GreaterThanOrEqual"],
        TOKEN_TYPE["Equality"],
        TOKEN_TYPE["Equal"],
        TOKEN_TYPE["And"],
        TOKEN_TYPE["Or"],
    ]


def call(parser):
    res = simple(parser)
    if (
        parser.peek_token_type() == TOKEN_TYPE["LeftParen"]
        or parser.peek_token_type() == TOKEN_TYPE["LeftBracket"]
    ):
        chain = []
        while (
            parser.peek_token_type() == TOKEN_TYPE["LeftParen"]
            or parser.peek_token_type() == TOKEN_TYPE["LeftBracket"]
        ):
            if parser.peek_token_type() == TOKEN_TYPE["LeftParen"]:
                parser.eat(TOKEN_TYPE["LeftParen"])
                args = expr_list(parser)
                parser.eat(TOKEN_TYPE["RightParen"])
                chain.append(new_call(args))
            else:
                parser.eat(TOKEN_TYPE["LeftBracket"])
                if parser.peek_token_type() == TOKEN_TYPE["Period"]:
                    parser.eat(TOKEN_TYPE["Period"])
                    id = parser.eat(parser.peek_token_type())
                    chain.append(new_attr(id["value"]))
                else:
                    chain.append(expr(parser))
                parser.eat(TOKEN_TYPE["RightBracket"])
        return new_chain(res, chain)
    return res


def expr(parser, wrapped=False):
    left = call(parser)
    if is_op(parser.peek_token()):
        op = parser.eat(parser.peek_token_type())["value"]
        right = expr(parser)
        return new_binop(left, right, op, wrapped)
    return left


def id_list(parser):
    # References in a group ()
    values = []
    if parser.peek_token_type() == TOKEN_TYPE["Word"]:
        values.append(parser.eat(TOKEN_TYPE["Word"])["value"])
        while parser.peek_token_type() == TOKEN_TYPE["Comma"]:
            parser.eat(TOKEN_TYPE["Comma"])
            values.append(parser.eat(TOKEN_TYPE["Word"])["value"])
    return values


def expr_list(parser):
    # Expressions in a group ()
    exprs = []
    if parser.peek_token_type() != TOKEN_TYPE["RightParen"]:
        exprs.append(expr(parser))
        while parser.peek_token_type() == TOKEN_TYPE["Comma"]:
            parser.eat(TOKEN_TYPE["Comma"])
            exprs.append(expr(parser))
    return exprs


def var_stmt(parser):
    parser.eat(TOKEN_TYPE["Var"])
    id = parser.eat(TOKEN_TYPE["Word"])["value"]
    parser.eat(TOKEN_TYPE["Equal"])
    value = expr(parser)
    return new_var(id, value)


def func_stmt(parser):
    parser.eat(TOKEN_TYPE["Func"])
    id = parser.eat(TOKEN_TYPE["Word"])["value"]
    parser.eat(TOKEN_TYPE["LeftParen"])
    params = id_list(parser)
    parser.eat(TOKEN_TYPE["RightParen"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_func(id, params, body)


def class_stmt(parser):
    parser.eat(TOKEN_TYPE["Class"])
    id = parser.eat(TOKEN_TYPE["Word"])["value"]
    parser.eat(TOKEN_TYPE["LeftBrace"])
    methods = []
    while parser.peek_token_type() == TOKEN_TYPE["Func"]:
        methods.append(func_stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_class(id, methods)


def return_stmt(parser):
    parser.eat(TOKEN_TYPE["Return"])
    value = expr(parser)
    return new_return(value)


def if_stmt(parser):
    parser.eat(TOKEN_TYPE["If"])
    parser.eat(TOKEN_TYPE["LeftParen"])
    condition = stmt(parser)
    parser.eat(TOKEN_TYPE["RightParen"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    otherwise = []
    if parser.peek_token_type() == TOKEN_TYPE["Else"]:
        otherwise.append(else_stmt(parser))
    while parser.peek_token_type() == TOKEN_TYPE["Elif"]:
        otherwise.append(elif_stmt(parser))
    if parser.peek_token_type() == TOKEN_TYPE["Else"]:
        otherwise.append(else_stmt(parser))
    return new_if(condition, body, otherwise)


def elif_stmt(parser):
    parser.eat(TOKEN_TYPE["Elif"])
    parser.eat(TOKEN_TYPE["LeftParen"])
    condition = stmt(parser)
    parser.eat(TOKEN_TYPE["RightParen"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_if(condition, body)


def else_stmt(parser):
    parser.eat(TOKEN_TYPE["Else"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_else(body)


def for_stmt(parser):
    parser.eat(TOKEN_TYPE["For"])
    id = parser.eat(TOKEN_TYPE["Word"])
    parser.eat(TOKEN_TYPE["Range"])
    parser.eat(TOKEN_TYPE["LeftParen"])
    through = expr_list(parser)
    parser.eat(TOKEN_TYPE["RightParen"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_for(id, through, body)


def while_stmt(parser):
    parser.eat(TOKEN_TYPE["While"])
    parser.eat(TOKEN_TYPE["LeftParen"])
    condition = expr(parser)
    parser.eat(TOKEN_TYPE["RightParen"])
    parser.eat(TOKEN_TYPE["LeftBrace"])
    body = []
    while parser.peek_token_type() != TOKEN_TYPE["RightBrace"]:
        body.append(stmt(parser))
    parser.eat(TOKEN_TYPE["RightBrace"])
    return new_while(condition, body)


def program(parser):
    parsed = []
    while parser.peek_token_type() != TOKEN_TYPE["Eof"]:
        parsed.append(stmt(parser))
    return parsed
