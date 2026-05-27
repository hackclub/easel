import argparse

from inu_interpreter import Interpreter
from inu_lexer import Lexer
from inu_parser import Parser
from inu_stdlib import inu_stdlib

parser = argparse.ArgumentParser(prog="inumaki", description="Inumkai programming language")
parser.add_argument("file", type=str, help="Inumaki source code file", nargs="?", default=None)

args = parser.parse_args()


def run(text):
    lexer = Lexer(text)
    lexer.scan_tokens()

    parser = Parser(lexer.tokens)
    parser.parse()

    interpreter = Interpreter(parser.ast, scope=inu_stdlib, cursed=0)
    interpreter.run()


if args.file:
    with open(args.file, "r") as file:
        text = file.read()

    run(text)
else:
    while True:
        try:
            text = input("inumaki> ")
        except (EOFError, KeyboardInterrupt):
            break

        run(text)
