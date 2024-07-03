# completely unfinished TODO

# Standard library reference

## Control flow

### if
(creates a new scope)

If the first argument is a true bool, evaluate the second arg, else evalueate the third one.
syntax:
```
(if (= 5 6)
    (print "condition is true")
    ~ else
    (print "condition is false")
)
```
The last argument is optional.

### for
(creates a new scope)

Defines the variable "varname" to 0, evaluates the body, increment varname, if varname < amount repeat
syntax:
```
(for varname (amount)
    body
)
```

Defines the variable "varname" to start, evaluates the body, increment varname, if varname < end repeat
syntax:
```
(for varname (start end)
    body
)
```

Defines the variable "varname" to start, evaluates the body, increment varname by increment, if varname < end repeat
syntax:
```
(for varname (start end increment)
    body
)
```

### repeat
