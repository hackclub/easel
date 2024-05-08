from aast import AST_TYPE
from pprint import pformat
from random import randint


kind = lambda value, kind: value["type"] == AST_TYPE[kind]


class Builtin:
    def _getattr(self, attr):
        return getattr(self, attr)


class Array(Builtin):
    def __init__(self, items):
        super().__init__()
        self.items = items

    def _get(self, index):
        return self.items[int(index)]

    def length(self):
        return len(self.items)

    def push(self, new):
        self.items.append(new)

    def update(self, index, value):
        self.items[int(index)] = value

    def __eq__(self, other):
        if isinstance(other, Array):
            return self.items == other.items
        return self.items == other

    def __repr__(self):
        return pformat(self.items)


class Dict(Builtin):
    def __init__(self, obj):
        super().__init__()
        self.obj = obj

    def _get(self, key):
        return self.obj[key]

    def update(self, key, value):
        self.obj[key] = value

    def __repr__(self):
        return pformat(self.obj)


def fire(*args):
    for arg in args:
        print(arg, end="")
    print()


def load(ask):
    # Wrapper for input
    return input(ask)


def random(min, max):
    return randint(min, max)


builtins = {
    "fire": fire,
    "load": load,
    "random": random,
    "int": int,
    "float": float,
    "str": str,
    "bool": bool,
}
