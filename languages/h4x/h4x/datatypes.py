import copy
from pprint import pprint
import json

import h4x

class BasicType:
	def __init__(self):
		self.type = "NULL"
	def __repr__(self):
		return self.type
	def copy(self):
		return copy.deepcopy(self)
class Value(BasicType):
	def __init__(self):
		self.type = "VALUE"
	def __repr__(self):
		return self.type + ":" + str(self.value)
	def __str__(self):
		return str(self.value)
class Null(Value):
	def __init__(self):
		self.type = "NULL"
		self.value = None
	def __repr__(self):
		return "Null"
	def __str__(self):
		return "Null"
class Number(Value):
	def __init__(self, value):
		self.type = "INTEGER"
		try:
			self.value = float(value)
		except ValueError:
			h4x.error.runtime(f"You can't make a number out of {repr(value)}")
	def __str__(self):
		return f'{self.value:g}'

class Bool(Value):
	def __init__(self, value):
		self.type = "BOOLEAN"
		self.value = value

class H4xList(Value):
	def __init__(self, value):
		self.type = "LIST"
		self.value = value

	def len(self):
		return len(self.value)
	def index(self, i):
		return self.value[int(i)].copy()
	def push(self, value):
		result = self.value[:]
		result.append(value.copy())
		return H4xList(result)
	def pop(self):
		result = self.value[:-1]
		return H4xList(result)
	def set(self, index, value):
		result = self.value[:]
		result[int(index)] = value.copy()
		return H4xList(result)

	def __str__(self):
		result = "("
		for i, elt in enumerate(self.value):
			result += str(elt)
			if i < len(self.value) - 1:
				result += ", "
		return result + ")"
	def __repr__(self):
		result = self.type  + "("
		for i, elt in enumerate(self.value):
			result += repr(elt) + ", "
		return result + ")"
class String(H4xList):
	def __init__(self, value):
		self.type = "STRING"
		self.value = value

	def index(self, i):
		return String(self.value[int(i)])
	def push(self, value):
		result = self.value + str(value.value) # TEMPORARY STRINGIFICATION
		return String(result)

	def __str__(self):
		return self.value
	def __repr__(self):
		return self.type + ":\"" + self.value + "\""

class Exec(BasicType):
	def __init__(self):
		self.type = "EXEC"

class EvaledExec(Exec):
	def __init__(self):
		self.type = "EVALED_EXEC"
class PyExec(EvaledExec):
	def __init__(self, function, num_args):
		self.type = "PY_EXEC"
		self.exec = function
		self.num_args = num_args
class H4xExec(EvaledExec):
	def __init__(self, arg_names, function):
		self.type = "H4X_EXEC"
		self.arg_names = arg_names
		self.num_args = len(arg_names)
		self.func_body = function
	def exec(self, args, scopes):
		scopes.append({})
		scopes[-1]["*trace"] = h4x.make_trace("function")
		for i, arg in enumerate(args):
			scopes[-1][self.arg_names[i]] = arg
		result = h4x.eval(self.func_body, scopes)
		scopes.pop()
		return result

class SpecialExec(Exec):
	def __init__(self, function):
		self.type = "SPECIAL_EXEC"
		self.exec = function