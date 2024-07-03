import h4x

exports = {}
def func_zoob(args, scopes):
	print("zoob")
	return h4x.datatypes.String("zoob")

exports["extras:zoob"] = h4x.datatypes.PyExec(func_zoob, 0)