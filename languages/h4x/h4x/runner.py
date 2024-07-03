import pprint

import h4x
from . import datatypes
from . import tokens
from . import error

def prettyprint(stuff):
	pretty = pprint.pformat(stuff)
	for line in pretty.split("\n"):
		print(">>|" * depth + line)

depth = -1

def eval(expr, scopes):
	global depth
	depth += 1
	if h4x.DEBUG:
		prettyprint(expr)

	if type(expr) != list:
		if "*trace" in scopes[-1]:
			scopes[-1]["*trace"]["token"] = expr
		h4x.DEBUG_last_token = expr
	h4x.DEBUG_scopes = scopes

	evaled = None
	if type(expr) == list:
		evaled = []
		if len(expr) > 0:
			first = eval(expr[0], scopes)
			evaled = [first]
			if isinstance(first, datatypes.Exec):
				if isinstance(first, datatypes.SpecialExec):
					evaled = first.exec(expr[1:], scopes)
				elif isinstance(first, datatypes.EvaledExec):
					for subexpr in expr[1:]:
						evaled.append( eval(subexpr, scopes) )
					
					if len(evaled) - 1 == first.num_args:
						evaled = first.exec(evaled[1:], scopes)
					else:
						error.runtime(f"{first} expected {first.num_args} argument but got {len(evaled) - 1}")
			else:
				for subexpr in expr[1:]:
					if isinstance(subexpr, datatypes.BasicType):
						evaled.append(subexpr)
					else:
						evaled.append( eval(subexpr, scopes) )
				if len(evaled) > 0:
					evaled = evaled[-1]
				
	elif expr.type == tokens.TokenTypes.STRING:
		evaled = datatypes.String(expr.data)

	elif expr.type == tokens.TokenTypes.NUMBER:
		if "." in expr.data:
			evaled = datatypes.Number(expr.data)
		else:
			evaled = datatypes.Number(expr.data)

	elif expr.type == tokens.TokenTypes.IDENTIFIER:
		for scope in reversed(scopes):
			if expr.data in scope:
				evaled = scope[expr.data]
				break
		if evaled == None:
			error.runtime(f'"{expr.data}" is not defined in any scope')
	elif isinstance(expr, datatypes.BasicType):
		return expr

	if h4x.DEBUG:
		prettyprint("returns")
		prettyprint(evaled)
		#input("")

	depth -= 1
	return evaled

