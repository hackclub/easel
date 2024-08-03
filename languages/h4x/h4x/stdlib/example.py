import h4x

exports = {}
def func_zoob(args, scopes):
	h4x.error.test_args(args, [h4x.datatypes.String], "example:tosty")
	print("tosty")
	return h4x.datatypes.String("tosty" + args[0])

exports["example:tosty"] = h4x.datatypes.PyExec(func_zoob, 1)
