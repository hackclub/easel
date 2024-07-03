import readline
import sys
from pprint import pprint

import h4x

h4x.DEBUG = False

scopes = h4x.create_scopes()
h4x.import_module(scopes[0], "h4x.stdlib")

def func_exit(args, scopes):
	print("Goodbye!")
	sys.exit()
def func_debug(args, scopes):
	h4x.DEBUG = not h4x.DEBUG
	if h4x.DEBUG:
		print("Debug mode activated")
	else:
		print("Debug mode disabled")

scopes[0]["exit"] = h4x.datatypes.PyExec(func_exit, 0)
scopes[0]["quit"] = scopes[0]["exit"]
scopes[0]["debug"] = h4x.datatypes.PyExec(func_debug, 0)

scopes[0][""] = h4x.datatypes.Null()

canceled = "="
while True:
	try:
		program = input(canceled + "> ") + "\n"
		tokenized = h4x.tokenize(program)
		while not h4x.parser.is_valid_program(tokenized):
			program += input(canceled + "] ") + "\n"
			tokenized = h4x.tokenize(program)
	except KeyboardInterrupt:
		print()
		canceled = "-"
		continue
	except h4x.error.H4xError:
		continue
	canceled = "="

	parsed = h4x.tokens_to_tree(tokenized)
	try:
		evaled = h4x.eval(parsed, scopes)
	except h4x.error.H4xError:
		evaled = "error"
	print("<" + canceled, end=" ")
	print(evaled)


#(define plus5 (fn (a) (+ a 5) ) )