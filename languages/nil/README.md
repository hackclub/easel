# NIL - The Non Intuitive Langauge

###### created as a project for HackClub's LangJam

NIL is a simple esoteric interpreted programming language written in rust. NIL code is written bottom to top, and reverses the typical syntax with with function calls wrapped in curly brackets and variables written value first this language is not meet to be easy to understand and let alone written

```
Hello World A Function Call and Returns

/*

;{output} {hello_world} ,Hello,

) {x} hello_world
  ;,World,
  ;{output} x
def (

*/
```

 

## Getting Started

#### Prerequisites

- rustc
- cargo

Step 1: Clone repo

```bash
git clone https://github.com/achester88/nil
```

```bash
cd nil
```

Step 2: Build via Cargo

```bash
cargo build --release
```

Step 3: Running NIL

```bash
cd target/release
```

Then make the NIL binary executable

```bash
chmod +x ./nil
```

Then to run it, enter:

```bash
./nil <file>
```

Read the docs here: [github](https://github.com/achester88/nil/blob/main/DOCS.md)



References
---

[LLVM tutorial in Rust language](https://github.com/jauhien/iron-kaleidoscope)

[A Gentle Introduction to LLVM IR](https://mcyoung.xyz/2023/08/01/llvm-ir/)
