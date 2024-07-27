This language was written to be difficult to program in, and as such I greatly encourage you to try and figure out the language by yourself before referencing these docs to get the true NIL experience

---

## Basics

NIL is written **bottom** to **top** to parsing with start with the end of the file and work its way up line by line. Any code that you want to be executed need to go in between a pair of `/* */`, anything outside of these pairs are consider comments and ignored by the lexer. Additional the completely unnecessary  ```;```  is mandatory and each statement must start with one.

## Types

The NIL language has 3 types: **str**, **bool**, and of course **num**

## Strings

Strings in NIL are surrounded, of course by the `,` symbol, which when compared to other typical languages allows for faster* creation of strings

```
,Hello NIL!,
```

* note: the use of commas has in no way be proved to be any faster the single or double quotes 

## Variables

Inorder to initialize variables in NIL the following syntax is used:

```rust
;name type
```

Type **has to** be ignored if a variable is initialize with a value

```rust
;value = name
```

Variables are all mutable by default so inorder the change the value the same syntax can be used ex:

```Rust
;count + 1 = count
;0 = count
```

## Control Flow

Seeing as almost every programming languages uses some form of the famous ```if then else``` statements NIL has chose to use a 100% original and different ```not if(nif) then else nif``` statements

```
)
    stantments...
) else nif !cond (
    stantments...
;nif cond (
```

this is the equivalent of the following js segment: 

```javascript
if (!cond) {
    stantments...
} else {
    stantments...
}
```

## Loops

To add to the 100% original innovations the NIL language as created are not loops or noops demonstrated below

```
)
    stantments...
;noop cond (
```

equivalent js code:

```js
while (!cond) {
    stantments...
}
```

## Functions

NIL has both built in and user definable functions, both of which can be called like so

```
;{fn_name} args
```

using the built-in output looks like this:

```
;{output} ,hello world!,
```

User Defined functions can be created like so:

```
) {args...} fn_name
    statments...
def (
```

* note:  the last value of the statement in the function is the return value

example of a function that squares a passed number and returns it 

```
) {x} sqaure
    ;x * x
def (
```

## Reference

### Built-In Functions

| Name      | Args | Return | Desc.                                   |
|:---------:|:----:|:------:|:---------------------------------------:|
| output    | ANY  | TRUE   | Output any value passed to the terminal |
| num_input | STR  | NUM    | Gets users input with optional prompt   |
| str_input | STR  | STR    | Gets users number with optional prompt  |
| round     | NUM  | NUM    | round passed number                     |

### Number Operations

| Op    | Args     | Return | Desc.                             |
| ----- | -------- | ------ | --------------------------------- |
| **+** | NUM, NUM | NUM    | Adds two numbers                  |
| **-** | NUM, NUM | NUM    | Subtracts two numbers             |
| **/** | NUM, NUM | NUM    | Divides two numbers               |
| *     | NUM, NUM | NUM    | Take a guess                      |
| **%** | NUM, NUM | NUM    | Returns the Remainder of division |

### String Operations

| Op  | Args | Return | Desc. |
| --- | ---- | ------ | ----- |

### Bool Operations

| Op     | Args       | Return | Desc.                           |
| ------ | ---------- | ------ | ------------------------------- |
| **==** | ANY*,ANY*  | BOOL   | Equal Operation                 |
| **!=** | ANY*,ANY*  | BOOL   | Not Equal Operation             |
| **&&** | BOOL, BOOL | BOOL   | OR Operation                    |
| **\|** | BOOL, BOOL | BOOL   | AND Operation                   |
| **>=** | NUM, NUM   | BOOL   | Greater Than or Equal Operation |
| **>**  | NUM, NUM   | BOOL   | Greater Than Operation          |
| **<=** | NUM, NUM   | BOOL   | Less Than or Equal Operation    |
| **>**  | NUM, NUM   | BOOL   | Less Than Operation             |

- note: the and and or operation have be swapped for the sole purpose of causing confusion

---
