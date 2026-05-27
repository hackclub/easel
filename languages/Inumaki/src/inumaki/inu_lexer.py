TOKENS = {
    "LeftParen": "LeftParen",
    "RightParen": "RightParen",
    "LeftBrace": "LeftBrace",
    "RightBrace": "RightBrace",
    "LeftBracket": "LeftBracket",
    "RightBracket": "RightBracket",
    "Dot": "Dot",
    "Comma": "Comma",
    "Colon": "Colon",
    "Keyword": "Keyword",
    "Identifier": "Identifier",
    "String": "String",
    "Number": "Number",
    "Boolean": "Boolean",
    "Or": "Or",
    "Not": "Not",
    "And": "And",
    "Equiv": "Equiv",
    "NotEquiv": "NotEquiv",
    "Gt": "Gt",
    "Gte": "Gte",
    "Lt": "Lt",
    "Lte": "Lte",
    "Plus": "Plus",
    "Minus": "Minus",
    "Asterisk": "Asterisk",
    "Modulo": "Modulo",
    "Slash": "Slash",
    "Semicolon": "Semicolon",
    "EOF": "EOF",
    "Salmon": True,
    "Bonito_Flakes": False,
}

chars = {
    "(": TOKENS["LeftParen"],
    ")": TOKENS["RightParen"],
    "{": TOKENS["LeftBrace"],
    "}": TOKENS["RightBrace"],
    "[": TOKENS["LeftBracket"],
    "]": TOKENS["RightBracket"],
    ".": TOKENS["Dot"],
    ",": TOKENS["Comma"],
    ":": TOKENS["Colon"],
    "+": TOKENS["Plus"],
    "-": TOKENS["Minus"],
    "*": TOKENS["Asterisk"],
    "/": TOKENS["Slash"],
    "%": TOKENS["Modulo"],
    "<": TOKENS["Lt"],
    "<=": TOKENS["Lte"],
    ">": TOKENS["Gt"],
    ">=": TOKENS["Gte"],
    "==": TOKENS["Equiv"],
    ";": TOKENS["Semicolon"],
}

KEYWORDS = [
    "Tuna",  # variables
    "Tuna_Mayo",  # functions
    "Return",
    "Mustard_Leaf",  # conditionals
    "Explode",
    "Twist",  # for loop
    "Plummet",  # while loop
    "Cough_Syrup",
]

CURSED_WORDS = [
    "EXPLODE",
    "TWIST",
    "CRUSH",
    "PLUMMET",
    "STOP",
    "SLEEP",
    "RETURN",
    "RUN",
    "BLAST",
]


class Token:
    def __init__(self, type, value, content, line, column):
        self.type = type
        self.value = value
        self.content = content
        self.line = line
        self.column = column

        self.cursed = self.value.upper() in CURSED_WORDS

    def __str__(self):
        return self.value


class Lexer:
    def __init__(self, text):
        self.text = text
        self.tokens = []
        self.current = 0
        self.line = 1
        self.column = 0

    def peek(self):
        if self.current >= len(self.text):
            return None
        return self.text[self.current]

    def peekn(self, n):
        if self.current + n >= len(self.text):
            return None
        return self.text[self.current + n]

    def advance(self):
        if self.current >= len(self.text):
            return None
        self.current += 1
        self.column += 1
        return self.text[self.current - 1]

    def match(self, char):
        if self.peek() == char:
            return self.advance()
        return False

    def match_word(self, word):
        for i in range(len(word)):
            if self.peekn(i) != word[i]:
                return False

        for _ in range(len(word)):
            self.advance()
        return True

    def start_word(self, char):
        word = char
        column = self.column
        while self.peek() and (self.peek().isalnum() or self.peek() == "_"):
            word += self.advance()

        match word:
            case "Or" | "Not" | "And":
                self.tokens.append(Token(TOKENS[word], word, word, self.line, column))
                return
            case "Kelp":
                while self.peek() and self.peek() != "\n":
                    self.advance()
                return
            case "Salmon" | "Bonito_Flakes":
                self.tokens.append(Token(TOKENS["Boolean"], word, word == "Salmon", self.line, column))
                return

        if word in KEYWORDS:
            self.tokens.append(Token(TOKENS["Keyword"], word, word, self.line, column))
        else:
            self.tokens.append(Token(TOKENS["Identifier"], word, word, self.line, column))

    def scan_token(self):
        char = self.advance()

        match char:
            case "(" | ")" | "{" | "}" | "[" | "]" | "." | "," | ":" | "+" | "-" | "*" | "/" | "%" | ";":
                self.tokens.append(Token(chars[char], char, char, self.line, self.column))
            case "<" | ">":
                if self.match("="):
                    self.tokens.append(Token(chars[char + "="], char + "=", char + "=", self.line, self.column))
                else:
                    self.tokens.append(Token(chars[char], char, char, self.line, self.column))
            case "=":
                if self.match("="):
                    self.tokens.append(Token(TOKENS["Equiv"], "==", "==", self.line, self.column))
                else:
                    raise Exception(f"Invalid character '=' at line {self.line}")
            case "'" | '"':
                string = ""
                while self.peek() != char:
                    string += self.advance()
                    if not self.peek():
                        raise Exception(f"Unterminated string at line {self.line}")

                self.advance()  # closing quote
                self.tokens.append(Token(TOKENS["String"], string, string, self.line, self.column))
            case " " | "\t" | "\r":
                pass
            case "\n":
                self.line += 1
                self.column = 0
            case _:
                if char.isdigit():
                    number = char
                    while self.peek() and (self.peek().isdigit() or (self.peek() == "." and self.peekn(1).isdigit())):
                        number += self.advance()

                    self.tokens.append(Token(TOKENS["Number"], number, float(number), self.line, self.column))
                elif char.isalpha() or char == "_":  # Gather the whole word
                    self.start_word(char)
                else:
                    raise Exception(f"Invalid character '{char}' at line {self.line}")

    def scan_tokens(self):
        while self.peek():
            self.scan_token()

        self.tokens.append(Token(TOKENS["EOF"], "EOF", None, self.line, self.column))

        return self.tokens
