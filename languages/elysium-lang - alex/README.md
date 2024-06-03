# Elysium Language

Welcome to **Elysium Language**, a programming language inspired by the elegance and simplicity of Python, the structure of JavaScript, and the functional aspects of Lisp. Elysium is designed for those who appreciate readability, maintainability, and the joy of writing clean code.

## Inspiration

The idea for Elysium Language came to me while working on various projects that required different programming paradigms. I found myself often wishing for a language that combined the best features of my favorite languages:

- **Python's readability and simplicity**: Python's clean syntax and readability have always made coding a pleasant experience for me. I wanted Elysium to reflect that simplicity.
- **JavaScript's versatility**: JavaScript's ability to handle both object-oriented and functional programming inspired the versatile nature of Elysium.
- **Lisp's powerful macro system**: The functional programming capabilities and the macro system of Lisp provided a powerful paradigm that I wanted to incorporate into Elysium.

## Features

- **Clean and Readable Syntax**: Elysium emphasizes code readability and simplicity, making it easy to learn and use.
- **First-Class Functions**: Functions in Elysium are first-class citizens, allowing you to pass them around and use them as arguments.
- **Versatile Paradigms**: Whether you prefer object-oriented programming, functional programming, or a mix of both, Elysium has you covered.
- **Dynamic Typing**: Similar to Python and JavaScript, Elysium uses dynamic typing, allowing you to write flexible and adaptable code.
- **Comprehensive Standard Library**: Elysium comes with a rich standard library that provides essential tools and utilities for everyday programming tasks.

## Installation

To get started with Elysium, you'll need to clone the repository and install the necessary dependencies. Make sure you have Node.js installed on your machine.

1. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/elysium-lang.git
    cd elysium-lang
    ```

2. **Install dependencies**:
    ```sh
    npm install
    ```

3. **Run your first Elysium program**:
    ```sh
    node src/index.js
    ```

## Example Code

Here's a simple Elysium program to give you a taste of what the language looks like:

```
let x = 10
let y = 20

if x < y {
    print x;
} else {
    print y;
}

while x > 0 {
    print x;
    x = x - 1;
}
```

## Language Syntax
# Variables
Variables are declared using the let keyword:

```
let x = 10
let y = x + 1
```

# Functions
# Note: this is a work in progress feature and is super buggy at the moment!!!!
Functions are defined using the function keyword:

```
function add(a, b) {
    return a + b;
}
```

# Control Structures
Elysium supports standard control structures such as if, else, and while:

```
if condition {
    // code block
} else {
    // code block
}

while x > 5 {
    // code block
}
```

# Contributing
I welcome contributions from anyone who is passionate about making Elysium better. Whether it's bug fixes, new features, or documentation improvements, your help is appreciated. Please fork the repository, make your changes, and submit a pull request.

# Contact
If you have any questions, suggestions, or feedback, feel free to reach out to me at alexanderli@hotmail.ca. You can also contact me on slack @alexanderli for updates on Elysium and other projects.

Thank you for your interest in Elysium Language. Happy coding!

Elysium Language is a personal project made with ❤️ by Alex.
