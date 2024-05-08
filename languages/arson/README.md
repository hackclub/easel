## What's this?

A programming language built for fun (no speed optimizations, probably has bugs, etc.)... around arson. (It's a joke though. Seriously.)

Usage: `python3 arson.py <program>.ars` (yes, I know, `ars`, haha).

**Make sure you have the latest version of Python! I'm using the lovely new keyword `match`.**

Here's a demo (skip towards the end) of me writing a Sudoku puzzle solver in Arson:

[![asciicast](https://asciinema.org/a/590145.svg)](https://asciinema.org/a/590145)

## Syntax

`/examples/readme/example.ars`:

```
burn x = 0 # Variable
for i through (0, 10) {
    burn x = x + 1 # No prefix notation currently
}

burn y = True
fire(y) # => True

prepmatch countdown(num) {
    # This is a function. Function declarations start with "prepmatch".
    while (num > 0) {
        fire(num) # fire = "print" in every other language
        burn num = num - 1
    }
}

countdown(x)

prepmatch power(num, exp) {
    # Basic power function
    burn res = 1
    for i through (0, exp) {
        burn res = res * num
    }
    return res
}

burn ans = power(8, 3)
fire(ans) # => 512

# PEMDAS
fire(power(8, 2) + (19 - 8) * (10 + 4))  # => 218

prepmatch min(a, b) {
    # This is an example of an if/else statement
    if (a < b) {
        return a
    } else {
        return b
    }
}

fire(min(2, -1)) # => -1

burn coords = [43.55, 42.55]
fire(coords) # => [43.55, 42.55]
fire(coords[.length]) # => 2

for i through (0, coords[.length]) {
    fire("index: ", i)
    fire(coords[i])
}

# Attributes start with . vs. references
coords[.push](power(8, 2) + (19 - 8) * (10 + 4))
fire(coords)
coords[.push]([1, 2])
fire(coords)

coords[3][.update](1, coords[3][1] * 2)
fire(coords) # => [43.55, 42.55, 218, [1, 4]]

# What about dictionaries
burn todo = {
    "1": "Burn",
    "2": "Clean",
    "3": "Escape"
}

fire(todo)
fire(todo["1"])  # => Burn

todo[.update]("2", [1, 2, {"a": "foo", "b": "bar"}])
fire(todo["2"][2]["b"])
```

I probably could have used more arson-related keywords. Or probably inserted a fire emoji here or there. But all of the current keywords revolve around being a pyromaniac, so +1 for that in my humble opinion.

### Also, it's extensible! 

Okay, what I mean is that you can create your own files of "builtins", I suppose, that you write in Python. For example, this is a snippet from `abuiltins.py`, which contains the code for the class `Dict`:

```python
class Dict(Builtin):
    def __init__(self, obj):
        super().__init__()
        self.obj = obj

    def _get(self, key):
        """
            Every builtin class has a _get "private" method.
        """
        return self.obj[key]

    def update(self, key, value):
        self.obj[key] = value

    def __repr__(self):
        """
            And a __repr__ method of course.
        """
        return pformat(self.obj)
```

## How it works

1. Source
2. Lexer creates tokens based on source
3. Parser creates AST based on tokens
4. Evaluator runs the AST, accounting for scope

## Quirks I noticed with working with Python

This is coming from someone who's worked primarily with JavaScript. (Although Python was my first language.)

* I can't apply it to a reference value, e.g. `TOKEN_TYPE["Word"]`, when using `match`.
* Had to rename my Python files to start with `a` because they were interfering with actual Python files, e.g. `ast.py`.
