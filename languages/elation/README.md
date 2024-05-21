# elation
A programming language with very simple syntax! Originally designed for Hack Club's programming language jam, Easel. 

Usage: `cargo run -- <program>.elation`

Or to run with a standalone executable of elation:
```
rustc src/main.rs -o elation
./elation <program>.elation
```

This is a demo video on how to use some of the basic features of elation.

[![asciicast](https://asciinema.org/a/4I19bktE7M5C7eKBUgNkdMC0t.svg)](https://asciinema.org/a/4I19bktE7M5C7eKBUgNkdMC0t)

## Syntax
Newlines separate instructions from each other and spaces separate the arguments provided on each line. Loops and if statements are made using jumps and labels.

| instruction (argument0) | argument1 | argument2                        | argument3      | argument4 |
|-------------------------|-----------|----------------------------------|----------------|-----------|
| calculate               | argument1 | operator (+ - * / % ^)           | argument2      | result    |
| compare                 | argument1 | operator (&& || == != > < >= <=) | argument2      | result    |
| concat                  | argument1 | argument2                        | result         |           |
| data                    | name      | content                          |                |           |
| exit                    |           |                                  |                |           |
| jump                    | label     |                                  |                |           |
| jump_if                 | boolean   | label_if_true                    |                |           |
| jump_if_else            | boolean   | label_if_true                    | label_if_false |           |
| label                   | name      |                                  |                |           |
| print                   | data      |                                  |                |           |
| read                    | result    |                                  |                |           |

`argument1` and `argument2` are both the name of variables to calculate or compare with. `result` is the name of the variable where the result will be stored. `data` is how variables are referred to in elation. Elation does not support spaces in strings as they are a part of the language syntax so you should use a replacement character such as _ instead. `read` is used to get user input. Elation reads and writes to the terminal line by line and does not allow for the printing and/or reading of individual characters without going on to a new line.

## Example
`/examples/fizzbuzz.elation`:

```
data fizz Fizz
data buzz Buzz
concat fizz buzz fizzbuzz
data zero 0
data one 1
data three 3
data five 5
data i 1
data max 100
label loop
compare i > max loop_over
jump_if loop_over end
calculate i % three remainder1
compare remainder1 == zero three_divisible
calculate i % five remainder2
compare remainder2 == zero five_divisible
compare three_divisible && five_divisible both_divisible
jump_if both_divisible fizzbuzz
jump_if three_divisible fizz
jump_if five_divisible buzz
print i
calculate i + one i
jump loop
label fizzbuzz
print fizzbuzz
calculate i + one i
jump loop
label fizz
print fizz
calculate i + one i
jump loop
label buzz
print buzz
calculate i + one i
jump loop
label end
exit
```

## How it works

1. Stores all labels in a vector
2. Runs through the program line by line
3. Splits each line on whitespace to separate the arguments
4. Evaluates the instruction based on the first argument on each line
