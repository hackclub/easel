# About Treestack
Treestack is a language built around the idea of having memory as a tree. Treestack works similarly to most stack languages, except that each item on the stack has an item of its own. Treestack also allows for much more movement than a typical stack language, with operators for both descending and ascending the tree, but moving along stacks. It also supports pointers, for easy storage of important values or spaces in memory. As treestack isn't as rigid in how the stack can be dealt with, it also supports some features that are more functional or array-lang like. It supports mapping, filtering, and metaprogramming through the allowance of evaluating strings as programs.

Check out some of the examples in the ./examples directory, any of them can be run with `treestack file` if you have it compiled, or just `cargo run -- file`.
Debug is also available through `-d` or `--debug`.
Running the lang without a file opens a repl.

A demo is also viewable at [Asciinema](https://asciinema.org/a/olzzI01svZEg1vmQHPjHm6uMu), but i recommend trying out the lang yourself.

# Syntax
The syntax is very simple, and is very close to forth with its reverse polish packer notation. It is is derived of 3-4 main types: [operators](operators.md), [words](words.md), and pushing; through raw number literals, string literals and char literals. There is also some control flow that acts a bit differently, which is described in [words](words.md).

A sample program may look like the below:
```
fn square { dup * }

1 100 range "square" map

while { . }
```
This program produces the first 100 squares, and prints them
