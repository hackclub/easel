import h4x
from h4x import datatypes

def func_upper(args, scopes):
	h4x.error.test_args(args, [datatypes.String], "str:upper")
	return h4x.datatypes.String(args[0].value.upper())
def func_lower(args, scopes):
	h4x.error.test_args(args, [datatypes.String], "str:lower")
	return h4x.datatypes.String(args[0].value.lower())
def func_str(args, scopes):
	return h4x.datatypes.String(str(args[0]))

def func_eq(args, scopes):
	h4x.error.test_args(args, [datatypes.String, datatypes.String], "str:eq")
	return h4x.datatypes.Bool(args[0].value == args[1].value)

def func_format(args, scopes):
	evaled = []
	for arg in args:
		evaled.append(h4x.eval(arg, scopes))
	if len(evaled) < 1:
		h4x.error.runtime(f"str:fmt expected at least 1 argument, but it got {len(args)}")
	if not isinstance(evaled[0], h4x.datatypes.String):
		h4x.error.runtime(f"The first argument to str:fmt should be a string, but it got {repr(evaled[0])}")
	formatted = ""
	formatters = evaled[1:]
	to_format = evaled[0]
	format_index = 0

	skip = False
	for i in range(to_format.len()):
		char = str(to_format.index(i))
		if char == "%":
			if i < (to_format.len() - 1) and str(to_format.index(i+1)) == "%":
				skip = True
				formatted += "%"
			elif skip:
				skip = False
			elif format_index < len(formatters):
				formatted += str(formatters[format_index])
				format_index += 1
			else:
				h4x.error.runtime(f"str:fmt got more % than formatters")
		else:
			formatted += char
	return h4x.datatypes.String(formatted)


exports = {}
exports["str:upper"] =  h4x.datatypes.PyExec(func_upper, 1)
exports["str:lower"] =  h4x.datatypes.PyExec(func_lower, 1)
exports["str:eq"] =     h4x.datatypes.PyExec(func_eq, 2)
exports["str:to_str"] = h4x.datatypes.PyExec(func_str, 1)
exports["str:fmt"] =    h4x.datatypes.SpecialExec(func_format)
