from pprint import pprint

import h4x
from h4x import datatypes

exports = {}

def func_num(args, scopes):
	return h4x.datatypes.Number(args[0].value)
def func_int(args, scopes):
	try:
		return h4x.datatypes.Number(int(float(args[0].value)))
	except ValueError:
		h4x.error.runtime(f"You can't make an int out of {repr(args[0])}")

# //---BASIC MATH---\\ #
def func_add(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "+")
	return h4x.datatypes.Number(args[0].value + args[1].value)
def func_sub(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "-")
	return h4x.datatypes.Number(args[0].value - args[1].value)
def func_mul(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "*")
	return h4x.datatypes.Number(args[0].value * args[1].value)
def func_div(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "/")
	return h4x.datatypes.Number(args[0].value / args[1].value)

def func_mod(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "mod")
	return h4x.datatypes.Number(args[0].value % args[1].value)

# //---COMARITION---\\ #
def func_eq(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "=")
	return h4x.datatypes.Bool(args[0].value == args[1].value)
def func_neq(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "!=")
	return h4x.datatypes.Bool(args[0].value != args[1].value)
def func_lt(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "<")
	return h4x.datatypes.Bool(args[0].value < args[1].value)
def func_gt(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], ">")
	return h4x.datatypes.Bool(args[0].value > args[1].value)
def func_lt_eq(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], "<=")
	return h4x.datatypes.Bool(args[0].value <= args[1].value)
def func_gt_eq(args, scopes):
	h4x.error.test_args(args, [datatypes.Number, datatypes.Number], ">=")
	return h4x.datatypes.Bool(args[0].value >= args[1].value)

# //---BOOLEAN---\\ #
def func_not(args, scopes):
	h4x.error.test_args(args, [datatypes.Bool], "not")
	return h4x.datatypes.Bool(not args[0].value)
def func_and(args, scopes):
	h4x.error.test_args(args, [datatypes.Bool, datatypes.Bool], "and")
	return h4x.datatypes.Bool(args[0].value and args[1].value)
def func_or(args, scopes):
	h4x.error.test_args(args, [datatypes.Bool, datatypes.Bool], "or")
	return h4x.datatypes.Bool(args[0].value or args[1].value)


exports["num"] = h4x.datatypes.PyExec(func_num, 1)
exports["int"] = h4x.datatypes.PyExec(func_int, 1)

exports["+"] =   h4x.datatypes.PyExec(func_add, 2)
exports["-"] =   h4x.datatypes.PyExec(func_sub, 2)
exports["*"] =   h4x.datatypes.PyExec(func_mul, 2)
exports["/"] =   h4x.datatypes.PyExec(func_div, 2)

exports["mod"] =   h4x.datatypes.PyExec(func_mod, 2)

exports["=="] =  h4x.datatypes.PyExec(func_eq, 2)
exports["="] =   h4x.datatypes.PyExec(func_eq, 2)
exports["!="] =  h4x.datatypes.PyExec(func_neq, 2)
exports["<"] =   h4x.datatypes.PyExec(func_lt, 2)
exports[">"] =   h4x.datatypes.PyExec(func_gt, 2)
exports["<="] =  h4x.datatypes.PyExec(func_lt_eq, 2)
exports[">="] =  h4x.datatypes.PyExec(func_gt_eq, 2)

exports["not"] = h4x.datatypes.PyExec(func_not, 1)
exports["and"] = h4x.datatypes.PyExec(func_and, 2)
exports["or"] =  h4x.datatypes.PyExec(func_or, 2)