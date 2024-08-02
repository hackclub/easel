## Meow! How does this work?

this is a fun and absolute pain of a language to use, but it was made for langjam!

I decided to follow the easel tutorial and changed some things up 

a lot of credit goes to jc and hack club!

the most noticeable of course is making it cat themed :3

be sure to read But Wait! since itll have some information if your code just wont work since a mysterious cat ate it....

anyways heres a video

[![asciicast](https://asciinema.org/a/VHYIf8TtPgA7VEMOw5CqMs1UD.svg)](https://asciinema.org/a/VHYIf8TtPgA7VEMOw5CqMs1UD)


### Defining a Variable

`meow x meoow 4` this creates the variable x 

there is no need to define variable types

it pretty simple think of `meow` as let in javascript

and think of `meoow` as an equal sign 

### Curly Brackets Got Changed >:)))

now you'll have to use `Meow` and `meoW` as left and right curly brackets respectively

### Print

`grrr("meow")`

### Random

`randmeow(min, max)`

returns an int that includes the min and max

### Conditionals 

example code: 

```
meow x meoow randmeow(0,2)

mrow (x == 0) Meow
    grrr("meow?")
meoW mroow (x == 1) Meow
    grrr("meow!")
meoW mroooww Meow
    grrr("meow")
meoW
```

here you can see the start of how weird this language looks like

the basic gist is: 

|  keyword| function |
| -----: | ----- |
|  mrow   | if|
|   mroow  | if else|
|  mroooww   | else|

### While Loops

while loops are pretty simple you just need to type in `mmeeooww` instead of `while`

### For Loops

for loops are a bit special since they take in the same conditionals as easel for loops

it will be a range with the first starting number then it will go up to the end

so not including the end

example:

```
meeow i meeeow (0,4) Meow
    grrr(i)
meoW
```

this should output 0, 1, 2, 3


### Defining a function

```
mmeow adding mmmeow (a, b) Meow
    prrr a + b
meoW
```

can also have a function without params

```
mmeow hi Meow
    grrr("hi")
meoW
```

|  keyword| function |
| -----: | ----- |
|  mmeow   | signals new function|
|   mmmeow  | just filler behind keyword |
|  prrr   | return |

### Arrays/Lists

`meow list meoow []` this creates a new list called list that is empty

you can the add to it by using `list.add("hi")`

then you can access things inside a list as usual with `list[0]` that should return `"hi"`

you can get the length by using `list.length`

### Structs

structs is used in easel but now its cat

`meeoow fruit meeooow Meow apples, oranges, cherries meoW`

this made the struct fruit with the different fuit inside it

you can then assign values to the things inside with `meow fruit.apples meoow 3`

we can then access it with `fruit.apples`

you can also use a struct as a basic structure (haha i get it now)

then make temporary versions of it and assign it to different places

such as:

```
meeoow x meeooow Meow a, b, c meoW

meow list meoow []

list.add(meeeoow x(a: 3, b: 4, c: 5))

grrr(list[0])
```

now list has a struct inside its first position


### Random stuff

i spent way too much time figuring out how to do this 

but i can do this now

```
meow x meoow 5
x += 5
grrr(x)
```

outputs 10

this also works with `++` and `--`

and also `-=`

## But Wait!

This language is quirky :3

remember you have to meow to the computer right?

well the computer now thinks its a cat and needs to be fed

you start out with `catHappy` set at 100

then it goes down a random ammount ranging from 0 to 5 for every line of code the parser reads and executes

BUT you can feed the cat with `feed()` that replenishes 3 to 15 happiness

you can also get the current `catHappy` by using `mow()` which returns the int

so you can print it out by using `grrr(mow())`

hope you got to this part in the read me and remember to use the `feed()` function or the cat will eat your code!!!