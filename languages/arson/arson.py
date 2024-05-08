from alexer import Lexer, scan_tokens
from aparser import Parser, program
from aeval import run
from json import dumps
from sys import argv, exit

OUTPUT_TO_FILE = True

if len(argv) != 2:
    print("Usage: python arson.py <file>")
    exit(1)

with open(argv[1]) as file:
    source = file.read()
    lexer = Lexer(source)
    scan_tokens(lexer)
    if OUTPUT_TO_FILE:
        with open("lexer.out", "w") as f:
            f.write(dumps(lexer.tokens, indent=4))
    parser = Parser(lexer.tokens)
    ast = program(parser)
    if OUTPUT_TO_FILE:
        with open("parser.out", "w") as f:
            f.write(dumps(ast, indent=4))
    run(ast)
