from pprint import pprint
import sys

import h4x

filename = "tests/stdlib.h4x"
if len(sys.argv) > 1:
	filename = sys.argv[1]

with open(filename, "r") as f:
	program = f.read()


scopes = h4x.create_scopes()
h4x.import_module(scopes[0], "h4x.stdlib")

try:
	parsed = h4x.parse(program)
	evaled = h4x.eval(parsed, scopes)
except h4x.error.H4xError:
	pass
#print(evaled)
