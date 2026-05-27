from inu_ast import (
    BinaryOp,
    Call,
    Conditional,
    For,
    Function,
    Get,
    Literal,
    Return,
    UnaryOp,
    Var,
    While,
    Set,
    CoughSyrup,
)
from inu_lexer import KEYWORDS, TOKENS, Token


class Parser:
    def __init__(self, tokens: list[Token]):
        self.tokens = tokens
        self.pos = 0
        self.ast = []

    def peek(self):
        if self.pos >= len(self.tokens):
            return None
        return self.tokens[self.pos]

    def peekn(self, n):
        if self.pos + n >= len(self.tokens):
            return None
        return self.tokens[self.pos + n]

    def eat(self, type):
        if type in TOKENS.keys():
            if (peek_type := self.peek().type) == type:
                self.pos += 1
                return self.tokens[self.pos - 1]
        elif type in KEYWORDS:
            if (peek_type := self.peek().value) == type:
                self.pos += 1
                return self.tokens[self.pos - 1]

        raise Exception(f"Expected {type}, got {peek_type}")

    def eat_keyword(self):
        if self.peek().type == TOKENS["Keyword"]:
            self.pos += 1
            return self.tokens[self.pos - 1]
        else:
            raise Exception(f"Expected keyword, got {self.peek().type}")

    def term(self):
        if self.peek().type == TOKENS["Identifier"]:
            var = self.eat("Identifier")
            name = Var(var.value, var.cursed)
            while self.peek().type in [TOKENS["Dot"], TOKENS["LeftParen"]]:
                if self.peek().type == TOKENS["Dot"]:
                    self.eat("Dot")
                    name = Get(name, self.eat("Identifier"))
                else:
                    self.eat("LeftParen")
                    args = []
                    while self.peek().type != TOKENS["RightParen"]:
                        args.append(self.expression())
                        if self.peek().type == TOKENS["Comma"]:
                            self.eat("Comma")
                    self.eat("RightParen")
                    name = Call(name, args)

            return name

        elif self.peek().type in [TOKENS["Number"], TOKENS["Boolean"], TOKENS["String"]]:
            literal = self.eat(self.peek().type)
            return Literal(literal.content, literal.cursed)
        elif self.peek().type == TOKENS["LeftParen"]:
            self.eat("LeftParen")
            expr = self.expression()
            self.eat("RightParen")
            return expr
        elif self.peek().type == TOKENS["Minus"]:
            self.eat("Minus")
            return UnaryOp("-", self.term())
        elif self.peek().type == TOKENS["Not"]:
            self.eat("Not")
            return UnaryOp("!", self.term())
        else:
            raise Exception(f"Unexpected token {self.peek().type} while processing term")

    def expression(self):
        left = self.term()

        while self.peek().type in [
            TOKENS["Plus"],
            TOKENS["Minus"],
            TOKENS["Asterisk"],
            TOKENS["Slash"],
            TOKENS["Modulo"],
            TOKENS["Equiv"],
            TOKENS["NotEquiv"],
            TOKENS["Lt"],
            TOKENS["Gt"],
            TOKENS["Lte"],
            TOKENS["Gte"],
            TOKENS["And"],
            TOKENS["Or"],
        ]:
            op = self.eat(self.peek().type)
            right = self.term()
            left = BinaryOp(left, op, right)

        return left

    def parse(self):
        while self.peek().type != TOKENS["EOF"]:
            self.ast.append(self.parse_statement())
        return self.ast

    def parse_statement(self):
        next = self.peek()

        if next.type == TOKENS["Keyword"]:
            match next.value:
                case "Tuna":
                    return self.assign_stmt()
                case "Tuna_Mayo":
                    return self.function_stmt()
                case "Return":
                    return self.return_stmt()
                case "Mustard_Leaf":
                    return self.conditional_stmt()
                case "Twist":
                    return self.for_stmt()
                case "Plummet":
                    return self.while_stmt()
                case "Cough_Syrup":
                    return self.cough_syrup()
                case _:
                    raise Exception(f"Unexpected keyword {next.value}")
        else:
            return self.expression()

    def assign_stmt(self):
        self.eat("Tuna")
        name = self.eat("Identifier")
        kw = self.eat_keyword()
        value = self.expression()

        cursed = name.cursed + kw.cursed

        return Set(name, value, cursed)

    def function_stmt(self):
        cursed = 0
        self.eat("Tuna_Mayo")
        name = self.eat("Identifier")
        cursed += name.cursed
        cursed += self.eat_keyword().cursed
        params = []
        while self.peek().type != TOKENS["Keyword"]:
            params.append(self.eat("Identifier"))
            cursed += params[-1].cursed
        cursed += self.eat_keyword().cursed

        self.eat("LeftBrace")
        body = []
        while self.peek().type != TOKENS["RightBrace"]:
            body.append(self.parse_statement())
        self.eat("RightBrace")

        return Function(name, params, body, cursed)

    def return_stmt(self):
        self.eat("Return")
        value = self.expression()

        return Return(value, cursed=1)

    def conditional_stmt(self):
        cursed = 0
        self.eat("Mustard_Leaf")
        cursed += self.eat_keyword().cursed
        condition = self.expression()
        cursed += self.eat_keyword().cursed
        self.eat("LeftBrace")
        body = []
        while self.peek().type != TOKENS["RightBrace"]:
            body.append(self.parse_statement())
        self.eat("RightBrace")
        else_body = None
        if self.peek().value == "Explode":
            self.eat("Explode")
            cursed += 1
            self.eat("LeftBrace")
            else_body = []
            while self.peek().type != TOKENS["RightBrace"]:
                else_body.append(self.parse_statement())
            self.eat("RightBrace")

        return Conditional(condition, body, else_body, cursed)

    def for_stmt(self):
        self.eat("Twist")
        cursed = 1
        cursed += self.eat_keyword().cursed
        var = self.parse_statement()
        if not isinstance(var, Set):
            raise Exception("Expected variable declaration in for loop")
        cursed += self.eat_keyword().cursed
        condition = self.expression()
        cursed += self.eat_keyword().cursed
        increment = self.parse_statement()
        cursed += self.eat_keyword().cursed

        self.eat("LeftBrace")
        body = []
        while self.peek().type != TOKENS["RightBrace"]:
            body.append(self.parse_statement())
        self.eat("RightBrace")

        return For(var, condition, increment, body, cursed)

    def while_stmt(self):
        self.eat("Plummet")
        cursed = 1
        cursed += self.eat_keyword().cursed
        condition = self.expression()
        cursed += self.eat_keyword().cursed
        self.eat("LeftBrace")
        body = []
        while self.peek().type != TOKENS["RightBrace"]:
            body.append(self.parse_statement())
        self.eat("RightBrace")

        return While(condition, body, cursed)

    def cough_syrup(self):
        self.eat("Cough_Syrup")
        return CoughSyrup()
