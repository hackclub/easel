# lambda calculus!

i almost certainly will mess up if i try to give an explanation, so please visit [the wikipedia page](https://en.wikipedia.org/wiki/Lambda_calculus)!

anyway, i believe this is the first lambda calculus programming language that aims to be an actual programming language. there are interpreters online, but all of those are either for math students or are impure (as in not completely functional). also, keep in mind that my verison has slightly different syntax:

functions: `λ{variable}.{expression}`

applications (the parenthesis are important!!): `({expression} {expression})`

variables: `{singular lowercase letter}`

there is also no combinator because i think other forms of recursion (such as the one shown in the factorial example program) are better. cheers!

to run, clone the repo and run `python3 λ.py examples/helloworld.λ` to run the helloworld program! that can be swapped out for any other programs you want to run.
