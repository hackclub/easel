# Iris
A language specializes in making a text-based adventure game!!!

## How to use
***Iris*** is incredibly easy to learn, within ***a few minutes***, you will able to make a text-based game ***yourself***!

### Basic syntax
**1. Paragraphs**

The most basic part of an ***Iris*** project is a paragraph.
```
Hello, welcome to Iris
```
This will output the content.

Texts on adjacent lines will be counted as one:
```
Iris
is awesome
```
It is the same as:
```
Iris is awesome
```

To have a break between your paragraphs, just have a break line between them:
```
This is the first paragraph!

This is the second one!
```

**2. Comments**

Text will be printed out by default unless it is a comment.

A comment is ignored by ***Iris*** so it wouldn't be printed:
```
# This is a comment, and this is unprintable!
```

**3. Sections**

Sections are the most important part of ***Iris***, for it is the structural unit of the game.

A section starts with a `-` (minus sign); normal paragraphs, comments, or choices are within every section.

```
- Section_one
Hello, welcome to this section
- Section_two
Zzz, Zzz, section two is sleeping Zzz, Zzz
```

Contents within sections won't be printed out unless you call that section. Call a section with a `>` (arrow sign), as follows:

```
1/1/2000
You are on the street, you really want to go home
> House
- House
"Welcome home, son" - your mother said
...
```

**4. Choices**

A choice is indicated by a `+` (plus sign). The choice will flow into the next instruction when you have chosen.

```
You see a cup of coffee
+ Drink it
You blacked out
> Die
+ Not drink it
Not thing happens
> Survive
```

### Advanced syntax
**1. Variables**
To declare a variable, you would use `~` (tilde sign), then the variable name, `=` (equal sign), and lastly the variable's value.

For example:
```
~ date = "1/1/2000"
```

You can also integrate Javascript within it:
```
~ sin = Math.sin
~ a = sin(10)
```

Variables can also be displayed within paragraphs choices, within this syntax:
```
~ role = "Wizard"
You are a ${role}!!!
```

**2. If statements**
Iris supports various features, including if statements. If statements, which are like variables, can be integrated with Javascript!

To start writing an if statement, you start with a `?` (question mark) and then your condition.

```
~ money = 1000
? money > 100 {
You are rich!
}
```

Also, remember to keep the close bracket in a new line, otherwise, it will cause an error!

```
# This will cause an error
? money > 100 {
You are rich! }

# But his won't
? money > 100 {
You are rich!
}
```

### Notes
***Notes:*** When you name a section, don't use space or any special characters, otherwise the game will cause errors!!!
***Notes:*** When you name a variable, don't use space or any special characters, otherwise the game will cause errors!!!

### Build
Run
```
node ./src/iris.js <your file(s) here>
```
It will create a new HTML file(s), open it up, and let's try out your new text-based game! 

🎉🎉🥳 Taddaaa!!! And you have learned how to make a game with Iris!!! 🎊🎊👏

### What makes Iris special
**Iris** is built with love and care. Iris specializes in making a text-based adventure game, with such easy syntax, you will be able to create a text-based game, that is what makes **Iris** special!!!

## Contact
Feel free to fork this repository or open issues. For any further information, please contact [my email](mailto:nguyengiabach1201@gmail.com).
