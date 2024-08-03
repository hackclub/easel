from pprint import pprint

import h4x

exports = {}


null = h4x.datatypes.Null()

# //---CONDITIONAL---\\ #
"""syntax
(if cond body_true optional_body_false)
"""
def func_if(args, scopes):
	if len(args) >= 2 and len(args) <= 3:
		condition = h4x.eval(args[0], scopes)
		if isinstance(condition, h4x.datatypes.Bool):
			if condition.value:
				scopes.append({})
				scopes[-1]["*trace"] = h4x.make_trace("if true") # create scope

				body_true = args[1]
				result = h4x.eval(body_true, scopes)

				scopes.pop()
				return result
			elif len(args) > 2:
				scopes.append({})
				scopes[-1]["*trace"] = h4x.make_trace("if false") # create scope
				body_false = args[2]
				result = h4x.eval(body_false, scopes)
				scopes.pop()
				return result
			else:
				return null
		else:
			h4x.error.runtime(f"the first argument to if must be a boolean, instead it got {repr(args[0])}")
	else:
		h4x.error.runtime(f"if needs at least 2 arguments, but it got {len(args)}")


# //---LOOPS---\\ #

"""syntax
(for var (start end increment) body)
(for var (start end) body)
(for var (amount) body)
"""
def func_for(args, scopes):
	if not len(args) >= 3:
		h4x.error.runtime(f"for needs at least 3 arguments, syntax: (for var (amount) body), instead it got {len(args)}")
	if not args[0].type == h4x.tokens.TokenTypes.IDENTIFIER:
		h4x.error.runtime(f"The first argument to for needs to be an identifier, instead it got {args[0].type}")

	varname = args[0].data
	loop = args[1]

	if type(loop) != list:
		h4x.error.runtime(f"The second argument to for should be in parenthesis, syntax (for var (amount) body), instead it got {loop}")
	if not (len(loop) >= 1 and len(loop) <= 3):
		h4x.error.runtime(f"The second argument to for should be a list with 1-3 elements in it, instead it got {len(loop)}")

	for i, elt in enumerate(loop):
		loop[i] = h4x.eval(elt, scopes)
	body = args[2:]

	scopes.append({}) # create scope
	scopes[-1]["*trace"] = h4x.make_trace("for")

	result = null

	start = loop[0] if len(loop) >= 2 else h4x.datatypes.Number(0)
	end = (loop[1].value if len(loop) >= 2 else loop[0].value)
	increment = loop[2].value if len(loop) == 3 else 1

	scopes[-1][varname] = start
	
	while True:
		if increment > 0:
			if scopes[-1][varname].value >= end:
				break
		else:
			if scopes[-1][varname].value <= end:
				break
			pass
		h4x.eval(body, scopes)
		scopes[-1][varname].value += increment

	scopes.pop()
	return result


"""syntax
(repeat amount body)
"""
def func_repeat(args, scopes):
	if not len(args) >= 2:
		h4x.error.runtime(f"repeat needs at least 2 arguments, syntax: (repeat amount body), instead it got {len(args)}")

	result = null
	amount = h4x.eval(args[0], scopes)

	if not isinstance(amount, h4x.datatypes.Number):
		h4x.error.runtime(f"The first argument to repeat must evaluate to a number, instead it evaluated to {repr(amount)}")
	
	scopes.append({}) # create scope
	scopes[-1]["*trace"] = h4x.make_trace("repeat")

	
	for i in range(int(amount.value)):
		result = h4x.eval(args[1:], scopes)
	scopes.pop()
	return result

"""syntax
(while cond body)
"""
def func_while(args, scopes):
	if not len(args) >= 2:
		h4x.error.runtime(f"while needs at least 2 arguments, syntax: (while cond body), instead it got {len(args)}")
	
	scopes.append({}) # create scope
	scopes[-1]["*trace"] = h4x.make_trace("while")
	
	result = null
	body = args[1:]
	while True:
		condition = h4x.eval(args[0], scopes)
		if not isinstance(condition, h4x.datatypes.Bool):
			h4x.error.runtime(f"The first argument to while must evaluate to a bool, instead it evaluated to {repr(condition)}")
		if not condition.value:
			break
		result = h4x.eval(body, scopes)
	
	scopes.pop()
	return result


exports["if"] =     h4x.datatypes.SpecialExec(func_if)
exports["for"] =    h4x.datatypes.SpecialExec(func_for)
exports["repeat"] = h4x.datatypes.SpecialExec(func_repeat)
exports["while"] =  h4x.datatypes.SpecialExec(func_while)