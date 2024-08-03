from pprint import pprint

import h4x
from h4x import datatypes

exports = {}
def func_list(args, scopes):
	evaled = []
	for arg in args:
		evaled.append(h4x.eval(arg, scopes))
	return h4x.datatypes.H4xList(evaled)

def func_len(args, scopes):
	h4x.error.test_args(args, [datatypes.H4xList], "#len")
	return h4x.datatypes.Number(args[0].len())
def func_index(args, scopes):
	h4x.error.test_args(args, [datatypes.H4xList, datatypes.Number], "#nth")
	return args[0].index(args[1].value)
def func_push(args, scopes):
	h4x.error.test_arg(args[0], datatypes.H4xList, "#push", 1)
	if not isinstance(args[0], h4x.datatypes.H4xList):
		h4x.error.runtime(f"The first argument to #push needs to be a H4xList, instead it got {repr(args[0])}")
	return args[0].push(args[1])
def func_pop(args, scopes):
	h4x.error.test_arg(args[0], datatypes.H4xList, "#pop", 1)
	return args[0].pop()
def func_set(args, scopes):
	h4x.error.test_args(args[:2], [datatypes.H4xList, datatypes.Number], "#set")
	return args[0].set(args[1].value, args[2].copy())

exports["#l"] = h4x.datatypes.SpecialExec(func_list)

exports["#len"] = h4x.datatypes.PyExec(func_len, 1)
exports["#nth"] = h4x.datatypes.PyExec(func_index, 2)
exports["#push"] = h4x.datatypes.PyExec(func_push, 2)
exports["#pop"] = h4x.datatypes.PyExec(func_pop, 1)
exports["#set"] = h4x.datatypes.PyExec(func_set, 3)