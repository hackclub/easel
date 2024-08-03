from pprint import pprint
import readline

import random
import h4x
from h4x import datatypes

exports = {}

null = h4x.datatypes.Null()

# //---BASIC STUFFS IG?---\\ #
def func_print(args, scopes):
	"""Prints a value"""
	result = null
	for arg in args:
		result = h4x.eval(arg, scopes)
		print( str(result), end=" ")
	print()
	return result
def func_input(args, scopes):
	try:
		return h4x.datatypes.String(input())
	except KeyboardInterrupt:
		h4x.error.runtime("The user canceled")
def func_int_input(args, scopes):
	result = None
	while result == None:
		try:
			result = int(input())
		except ValueError:
			pass
	return h4x.datatypes.Number(result)

def func_rand(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "rand")
	return h4x.datatypes.Number(random.randint(int(args[0].value), int(args[1].value)))

def func_prnt_scope(args, scopes):
	"""DEBUG prints the scopes"""
	pprint(scopes)
	return null

def func_error(args, scopes):
	h4x.error.test_args(args, [datatypes.String], "error")
	h4x.error.runtime(str(args[0]))

def func_py(args, scopes):
	h4x.error.test_args(args, [datatypes.String], "py")
	exec(args[0].value)

def func_type(args, scopes):
	"""Return the type of the argument"""
	return args[0].type

def func_do(args, scopes):
	"""Creates a new scope and runs the arguments in it"""
	scopes.append({})
	scopes[-1]["*trace"] = h4x.make_trace("do")
	result = h4x.eval(args, scopes)
	scopes.pop()
	return result

def func_import(args, scopes):
	"""Import a python module found in the PATH"""
	if not len(args) == 1:
		h4x.error.runtime(f"import needs 1 argument, instead it got {len(args)}")
	if not args[0].type == h4x.tokens.TokenTypes.IDENTIFIER:
		h4x.error.runtime(f"The first argument to import needs to be an identifier, instead it got {args[0]}")
	h4x.import_module(scopes[-1], args[0].data, args[0].data + ":")
	return h4x.datatypes.String(args[0].data)

def func_fn(args, scopes):
	"""Returns a function, the first argument is a list of parameters, the rest is the body of the function"""
	if not len(args) >= 2:
		h4x.error.runtime(f"fn needs at least 2 arguments, instead it got {len(args)}")
	if not type(args[0]) == list:
		h4x.error.runtime(f"the first argument to fn needs to be a list of arguments, instead it got {args[0]}")
	arg_names = []
	for name in args[0]:
		arg_names.append(name.data)
	body = args[1:]
	return h4x.datatypes.H4xExec(arg_names, body)


def func_define(args, scopes):
	"""Defines a variable in the current scope"""
	if not len(args) == 2:
		h4x.error.runtime(f"Define needs two arguments, syntax (define a 5). It got {len(args)}")
	if not args[0].type == h4x.tokens.TokenTypes.IDENTIFIER:
		h4x.error.runtime(f"The first parameter to define needs to be an identifier, instead it got a {args[0].type}")
	varname = args[0].data
	value = h4x.eval(args[1], scopes).copy()
	scopes[-1][varname] = value
	#return value
	return null
	
def func_set(args, scopes):
	"""Finds the variable in the closest scope and sets it to the second argument"""
	if not len(args) == 2:
		h4x.error.runtime(f"Set needs two arguments, syntax (set a 5). It got {len(args)}")
	if not args[0].type == h4x.tokens.TokenTypes.IDENTIFIER:
		h4x.error.runtime(f"The first parameter to set needs to be an identifier, instead it got a {args[0].type}")
	varname = args[0].data
	value = h4x.eval(args[1], scopes).copy()
	found_var = False
	for scope in reversed(scopes):
		if varname in scope:
			scope[varname] = value
			found_var = True
			break
	if not found_var:
		scopes[0][varname] = value
	#return value
	return null


exports["do"] =     h4x.datatypes.SpecialExec(func_do)
exports["import"] = h4x.datatypes.SpecialExec(func_import)
exports["fn"] =     h4x.datatypes.SpecialExec(func_fn)

exports["define"] = h4x.datatypes.SpecialExec(func_define)
exports["set"] =    h4x.datatypes.SpecialExec(func_set)

exports["print"] =  h4x.datatypes.SpecialExec(func_print)
exports["input"] =  h4x.datatypes.PyExec(func_input, 0)
exports["int_input"] =  h4x.datatypes.PyExec(func_int_input, 0)
exports["rand"] =   h4x.datatypes.PyExec(func_rand, 2)
exports["py"] =   h4x.datatypes.PyExec(func_py, 1)
exports["type"] =   h4x.datatypes.PyExec(func_type, 1)
exports["error"] =  h4x.datatypes.PyExec(func_error, 1)
exports["scopes"] = h4x.datatypes.PyExec(func_prnt_scope, 0)

exports["true"] =  h4x.datatypes.Bool(True)
exports["false"] = h4x.datatypes.Bool(False)
exports["null"] =  h4x.datatypes.Null()