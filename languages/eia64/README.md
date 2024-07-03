# Eia64

Git repo: [XomaDev/Eia64](https://github.com/XomaDev/Eia64)

Eia64 is an interpreted language that draws inspiration from the lovely syntax of Kotlin and many other languages like Go.

Look at asciinema recording [here](https://asciinema.org/a/666650).

Language is extensively documented at [themelon.space/eia](https://themelon.space/eia)

`/stdlib` is where the standard library of Eia is located, it supports `string`, `array` and some `math`.\
`/examples` are some really cool codes written in Eia 👀

## How to run

> Requires >= Java 11, to know your version run `java -version`

Clone the repository

```bash
git clone https://github.com/XomaDev/Eia64
```

### Live Mode

To enter into a live mode do `java -jar Eia64.jar live`

```kotlin
$ java -jar Eia64.jar live
> println("Hello, World!")
> ~~
Hello, World!
```

Type in the code in the terminal, to execute it type `~~` in a next line.\
Use `exit` to exit the terminal.

### Pass in a source file

`java -jar Eia64.jar <source_file>`

````kotlin
$ java -jar Eia64.jar examples/animation.eia
Enter a word: meow
🗑(> ^_^)>          meow
🗑 (> ^_^)>         meow
🗑  (> ^_^)>        meow
🗑   (> ^_^)>       meow
🗑    (> ^_^)>      meow
````

Enjoy!\
Crafted with Love ❤️\
Kumaraswamy B G • 16-year-old