<img src="https://raw.githubusercontent.com/commonkestrel/fateful/master/misc/fateful_icon.png" alt="Fateful logo" width="100" align="left" />

# Fateful
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/commonkestrel/fateful/rust.yml)

Fateful is a CLI tool foring with my homebrew CPU, F8ful.
It contains an [emulator](#emulator) and an [assembler](#assembler), as well as a full [test suite](#tests).
Fateful can be installed via [cargo](https://github.com/rust-lang/cargo): 
```bash 
cargo install --git https://github.com/commonkestrel/fateful
```

Running a program has two steps: assembly and emulation.
To assemble a program, run the `assemble` or `asm` command: 
```bash
fateful assemble <program>.asm -o <program>.bin
```

If this is successful, you can emulate the program with the `emulate` or `emu` command: 
```bash
fateful emulate <program>.bin
```

The emulator is a REPL that contains various commands expalined [below](#emulator).
The most important command for emulating a program is `RUN`.
Inputting `RUN 0` will run the assembly program as fast as possible until a halt is detected.

Here's a demo of the `screen.asm` example:

![Demo of the Fateful program](./misc/demo.gif)

## Assembler

The assembler can be used with the `fateful asm` or `fateful assemble` command to assemble fateful assembly into f8ful machine code.
The input and output are both optional, and default to `stdin` and `stdout` respectively.
Input is positional, being the first argument, and the output can be specified with the `-o` or `--output` flag.

### Instruction Set

Fateful assembly contains just 16 instructions,
including arithmetic, memory operations, a hardware stack, 
and a `jnz` (Jump if not zero) instruction making the CPU turing complete.

0. [ADD](#add)
0. [SUB](#sub)
0. [ADC](#adc)
0. [SBB](#sbb)
0. [NAND](#nand)
0. [OR](#or)
0. [CMP](#cmp)
0. [MV](#mv)
0. [LD](#ld)
0. [ST](#st)
0. [LDA](#lda)
0. [LPM](#lpm)
0. [PUSH](#push)
0. [POP](#pop)
0. [JNZ](#jnz)
0. [HALT](#halt)

#### ADD

Machine Code: `0x0`

Operation: Adds the first and second operand, storing the result in the first operand.

```asm
add <register>, <register/imm8>
```

#### SUB

Machine Code: `0x1`

Operation: Subtracts the second operand from the first, storing the result in the first operand.

```asm
sub <register>, <register/imm8>
```

#### ADC

Machine Code: `0x2`

Operation: Adds the first and second operands, plus an additional 1 if the carry bit is set, storing the result in the first operand.

```asm
adc <register>, <register/imm8>
```

#### SBB

Machine Code: `0x3`

Operation: Subtracts the second operand from the first, subtracting an additional 1 if the carry bit is set, storing the result in the first operand.

```asm
sbb <register>, <register/imm8>
```

#### NAND

Machine Code: `0x4`

Operation: Performs a bitwise NAND on the first and second operands, storing the result in the first operand.

```asm
nand <register>, <register/imm8>
```

#### OR

Machine Code: `0x5`

Operation: Performs a bitwise OR on the first and second operands,
storing the result in the first operand.

```asm
or <register>, <register/imm8>
```

#### CMP

Machine Code: `0x6`

Operation: Compares the first and second operands, storing the comparison results in the status register. 

```asm
cmp <register>, <register/imm8>
```

#### MV

Machine Code: `0x7`

Operation: Copies the second operand into the first operand.

```asm
mv <register>, <register/imm8>
```

#### LD

Machine Code: `0x8`

Operation: Loads the byte at either the RAM address provided, or the RAM address stored in the HL registers if none is provided, into the first operand.

```asm
ld <register>, [address]
```

#### ST

Machine Code: `0x9`

Operation: Stores the first operand into RAM at either the address provided, or the address stored in the HL registers if none is provided.

```asm
st [address,] <register>
```

#### LDA

Machine Code: `0xA`

Operation: Loads the provided 16-bit address into the HL registers.

```asm
lda <address>
```

#### LPM

Machine Code: `0xB`

Operation: Loads the byte at either the ROM address provided, or the ROM address stored in the HL registers if none is provided, into the first operand.

```asm
lpm <register>, [address]
```

#### PUSH

Machine Code: `0xC`

Operation: Stores the first operand to the RAM location currently pointed to by the stack pointer, then decrements the stack pointer. 

```asm
push <register/imm8>
```

#### POP

Machine Code: `0xD`

Operation: Increments the stack pointer, then loads the RAM location currently pointed to by the stack pointer into the first operand.

```asm
pop <register>
```

#### JNZ

Machine Code: `0xE`

Operation: Jumps to the address pointed to by the HL registers only if the first operand is not zero.

```asm
jnz <register/imm8>
```

#### HALT

Machine Code: `0xF`

Operation: Sets the H bit in the status register, halting the CPU

```asm
halt
```

### Labels

Labels help make blocks of your program easily accessible.
They consist of an identifier followed by a colon (`:`).
Labels with an identifier beginning with `.` are considered local to the most recent global label.
Local labels can also be accessed globally via `<parent>.<local>`.

Example:
```asm
parent:
    ; assembly code
.local1:
    jmp [.local1]
.local2:
    jmp [parent.local2]
```

### Literals

Both integer and string literals are valid in Fateful assembly.

#### Integers

Integer literals can be defined a few ways.
An immediate integer can be a decimal (`100`),
but they can also be defined with hexadecimal (`0x64`), octal (`0o144`), and binary (`0b01100100`).
Integers can also be defined with a character, surrounded by single-quotes (`'`).
Characters support the same escape sequences as [strings](#strings).
Expressions can be used to make it more clear where a value comes from.
Expressions must be able to be evaluated at compile-time, and are surrounded in parentheses.
Expressions support adding, subtracting, multiplying, dividing, modulus, common bit-wise operators,
and boolean expressions (with true as `1` and false as `0`).

There is also a special symbol that can be used in place of an integer: `$`.
The `$` symbol represents the value of the program counter at the start of the current instruction.
This can be very useful for calculating relative jumps, and can be used anywhere an integer literal can.

#### Strings

Strings can only be used with the `@str` directive, and are surrounded in double-quotes (`"`).
The compiler will automatically append a null-byte to every string literal.
Strings support a variety of escape sequences:

| Escape Sequence | Result                                  |
|-----------------|-----------------------------------------|
| `\n`            | Line Feed (`◙` in code-page 737)        |
| `\\`            | `\`                                     |
| `\"`            | `"`                                     |
| `\'`            | `'`                                     |
| `\0`            | Null character                          |
| `\v`            | Vertical Tab (`♂` in code-page 737)     |
| `\t`            | Horizontal Tab (`○` in code-page 737)   |
| `\r`            | Carriage return (`♪` in code-page 737)  |
| `\a`            | Bell (`•` in code-page 737)             |
| `\b`            | Backspace (`◘` in code-page 737)        |
| `\f`            | Form Feed (`♀` in code-page 737)        |
| `\xFF`          | 8-bit character code (exactly 2 digits) |
| `\o777`         | 8-bit character code (exactly 3 digits) |

### Preprocessor Directives

There are a variety of C-style preprocessor directives included in the assembler, indicated with a preceding `@`.
These directives can apply conditional transformations to the source before compilation.
Macros are processed in top-down order, meaning if a `@define` is placed below an `@ifdef` in the file, the define will not be in scope during the check.

#### DEFINE

The define macro links an identifier to a group of tokens.
Before compiling, each instance of this identifier is replaced with the specified tokens.

Unlike C, this does not support function-style definitions, meaning no arguments are allowed.

Syntax:
```rs
@define <identifier> <value>
```

#### UNDEF

The `@undef` macro removes (undefines) the current definition of the given identifier.
Consequently, subsequent occurrences of the identifier are ignored by the preprocessor.

Syntax:
```rs
@undef <identifier>
```

#### ERROR

The `@error` directive emits a user-specified error message before terminating the assembly.

Syntax:
```rs
@error "error message"
```

#### IF

The `@if` directive controls compilation of portions of a source file.
If the expression you write after the `@if` is greater than 0, the block following the `@if` is retained for assembly.

Syntax:
```
@if <expr>
    ...
@endif
```

#### ELIF

The `@elif` directive is only allowed as part of an `@if` block,
and is only evaluated if the previously evaluated blocks' check evaluates to 0.
Similar to the `@if` directive, if the expression you write after the `@elif` is greater than 0, the block following the `@elif` is retained for assembly.

Syntax:

```
@if <expr>
    ...
@elif <expr>
    ...
@endif
```

#### ELSE

The `@else` directive is only allowed at the end of an `@if` block.
If the expression of the previously evaluated block's check evaluates to 0,
then the block following the `@else` is retained for assembly.

Syntax:

```
@if <expr>
    ...
@else
    ...
@endif
```

#### IFDEF

The `@ifdef` directive is functionally the same as `@if 1` if the identifier has been defined,
and `@if 0` when the identifier hasn't been defined, or has been undefined by the `@undef` directive.

Syntax:
```
@ifdef <identifier>
    ...
@endif
```

#### IFNDEF

The `@ifndef` directive is functionally the same as `@if 0` if the identifier has been defined,
and `@if 1` when the identifier hasn't been defined, or has been undefined by the `@undef` directive.

Syntax:
```
@ifndef <identifier>
    ...
@endif
```

#### Include

The include macro pastes a stream of tokens from another file.
The file must be located in a package, and can be indexed by filepath relative to the root of the package.

A package is linked to an identifier through a rich comment, and can be either a local directory or a remote git repository.


Syntax:
```rs
/// <package> = <path/git repository>
@include <<package>/<file path>>
```

Example:

```rs
/// error = https://github.com/commonkestrel/f8ful_os
@include <error/error.asm>
```

### Segments

The assembly is divided into segments, specified with the `@cseg` and `@dseg` directives,
and organized by the `@org` directive.
The `@org` directive will apply to the current segment, and can only be specified once per segment.
Segments can be used to organize blocks of data and code throughout the address space.

#### Code Segments

Code segments, signified by the `@cseg` directive,
are where all of your assembly instructions are located.
Each assembly program starts in an initial code segment.

#### Data Segments

Data segments, signified by the `@dseg`,
are a block in RAM rather than the ROM.
These segments allow you to reserve blocks for global variables.
The variables defined in a data segment will reserve a RAM address while automatically avoiding collision.

Variables can be defined with a few directives.
These directives allow reserving blocks of variable size, specified here:
| Directive                  | Size   |
|----------------------------|--------|
| `@byte <identifier>`       | 1      |
| `@double <identifier>`     | 2      |
| `@quad <identifier>`       | 4      |
| `@var <size> <identifier>` | *size* |

These variables will resolve to an address at assembly, and can be accessed via *$identifier*.

#### Organization

Segments are automatically arranged to avoid collision,
but they can be manually organized with the `@org` directive.
This directive will place the origin of a segment at the address specified.
This is especially useful to make sure that the origni of your code is placed at `0x0000`,
since the program starts running from this point.

Unfortunately, manual organization can result in collisions,
so you must be careful to avoid these if manually organizing two or more segments of the same type.

### Data

You can place raw data within the program as well.
This data can be acessed with `lpm`.
Similar to variables in the data segment, these are placed with the following directives:

* `@byte <imm8>`
* `@double <imm16>`
* `@quad <imm32>`
* `@str <string>`

This data is often used in conjunction with a label in order to make it easily locatable.

Example
```c
hello:
    @str "hello world"
```

### Macros

Macros are an incredibly powerful part of this assembler, 
and are defined with the `@macro` directive.
They are similar to C's function-style `#define` macros,
but with optionally typed parameters and multiple definitions for different parameters.

Each parameter can have an accepted type, or multiple accepted types with the `|` operator.
Parameter identifiers must begin with a `%`.
These types are listed here:

* `reg`: Matches a register input (A, B, C, D, E, F, H, L)
* `imm`: Matches an immediate integer
* `addr`: Matches a RAM address
* `label`: Matches a ROM address
* `ident`: Matches any identifier
* `str`: Matches a string literal
* `any`: Matches any of the previous

Syntax:
```asm
; This syntax defines a singular signiture
@macro <identifier> (<parameters>) {
    ; assembly code
}

; This syntax allows for the definition of multiple signitures
@macro <identifier> {
    (<parameters>) {

    }

    (<parameters>) {

    }
}
```

This may be a little confusing, so we can use an example from the built-in macros (we'll get to these later):

```asm
@macro jmp {
    () {
        jnz 1
    }
    (%location:label) {
        lda %location
        jmp
    }
}
```

What are we even looking at here?
Well, this macro contains two signitures - 
one with an empty parameter list and one with a ROM address bound to the `%location` parameter.
As you can see, the second signiture contains another `jmp` instruction,
showcasing the fact that these macros are evaluated recursively.

Macros are used just like normal instructions.
For example, the `jmp` macro can be used like `jmp [foo]`,
which expands to this:

```asm
lda [foo]
jnz 1
```

### Built-in Macros

Built-in macros are a group of macros included by default in every program.
The details of each macro can be found in (src/assembler/macros.asm)[./src/assembler/macros.asm].

* [PUSH](#push-macro)
* [POP](#pop-macro)
* [PUSHA](#pusha-macro)
* [POPA](#popa-macro)
* [JMP](#jmp-macro)
* [JNZ](#jnz-macro)
* [JLT](#jlt-macro)
* [JLE](#jle-macro)
* [JGT](#jgt-macro)
* [JGE](#jge-macro)
* [JEQ](#jeq-macro)
* [JZ](#jz-macro)
* [CALL](#call-macro)
* [RET](#ret-macro)
* [MV16](#mv16-macro)
* [ADD16](#add16-macro)
* [SUB16](#sub16-macro)
* [INC](#inc-macro)
* [DEC](#dec-macro)
* [NOT](#not-macro)
* [AND](#and-macro)
* [XOR](#xor-macro)
* [SHL](#shl-macro)
* [NOP](#nop-macro)
* [USE](#use-macro)

#### PUSH Macro

```asm
push r0: reg|imm, r1: reg|imm
```
Pushes two values to the stack in ascending parameter order.

```asm
push r0: reg|imm, r1: reg|imm, r2: reg|imm
```
Pushes three values to the stack in ascending parameter order.

```asm
push r0: reg|imm, r1: reg|imm, r2: reg|imm, r3: reg|imm
```
Pushes four values to the stack in ascending parameter order.

```asm
push r0: reg|imm, r1: reg|imm, r2: reg|imm, r3: reg|imm, r4: reg|imm
```
Pushes five values to the stack in ascending parameter order.

```asm
push r0: reg|imm, r1: reg|imm, r2: reg|imm, r3: reg|imm, r4: reg|imm, r5: reg|imm
```
Pushes six values to the stack in ascending parameter order.

#### POP Macro

```asm
pop r0: reg|imm, r1: reg|imm
```
Pops two values from the stack in ascending parameter order.

```asm
pop r0: reg|imm, r1: reg|imm, r2: reg|imm
```
Pops three values from the stack in ascending parameter order.

```asm
pop r0: reg|imm, r1: reg|imm, r2: reg|imm, r3: reg|imm
```
Pops four values from the stack in ascending parameter order.

```asm
pop r0: reg|imm, r1: reg|imm, r2: reg|imm, r3: reg|imm, r4: reg|imm
```
Pops five values from the stack in ascending parameter order.

```asm
pop r0: reg|imm, r1: reg|imm, r2: reg|imm, r3: reg|imm, r4: reg|imm, r5: reg|imm
```
Pops six values from the stack in ascending parameter order.

#### PUSHA Macro

```asm
pusha
```
Pushes all six general-purpose registers to the stack in ascending order.
Designed to be paired with the `popa` macro.

#### POPA Macro

```asm
popa
```
Pops the top 6 values on the stack into the six general-purpose registers in decending order.
Designed to be paired with the `pusha` macro.

#### JMP Macro

```asm
jmp
```
Jumps to the location pointed to by the HL register unconditionally.

```asm
jmp location: label
```
Jumps to *location* unconditionally.

#### JNZ Macro

```asm
jnz condition: reg|imm, location:label
```
Jumps to *location* if *condition* is not zero.

#### JLT Macro

```asm
jlt
```
Jumps to the address pointed to by the HL registers if the `L` flag in the status register is set.

```asm
jlt location: label
```
Jumps to *location* if the `L` flag in the status register is set.

```asm
jlt x: reg, y: reg|imm
```
Jumps to the location pointed to by the HL registers if *x* < *y*.

```asm
jlt x: reg, y: reg|imm, location: label
```
Jumps to *location* if *x* < *y*.

#### JLE Macro

```asm
jle
```
Jumps to the address pointed to by the HL registers if the `L` or `E` flags in the status register are set.

```asm
jle location: label
```
Jumps to *location* if the `L` or `E` flags in the status register are set.

```asm
jle x: reg, y: reg|imm
```
Jumps to the location pointed to by the HL registers if *x* <= *y*.

```asm
jle x: reg, y: reg|imm, location: label
```
Jumps to *location* if *x* <= *y*.

#### JGT Macro

```asm
jgt
```
Jumps to the address pointed to by the HL registers if the `G` flag in the status register is set.

```asm
jgt location: label
```
Jumps to *location* if the `G` flag in the status register is set.

```asm
jgt x: reg, y: reg|imm
```
Jumps to the location pointed to by the HL registers if *x* > *y*.

```asm
jgt x: reg, y: reg|imm, location: label
```
Jumps to *location* if *x* > *y*.

#### JGE Macro

```asm
jge
```
Jumps to the address pointed to by the HL registers if the `G` or `E` flags in the status register are set.

```asm
jge location: label
```
Jumps to *location* if the `G` or `E` flags in the status register are set.

```asm
jge x: reg, y: reg|imm
```
Jumps to the location pointed to by the HL registers if *x* >= *y*.

```asm
jge x: reg, y: reg|imm, location: label
```
Jumps to *location* if *x* >= *y*.

#### JEQ Macro

```asm
jeq
```
Jumps to the address pointed to by the HL registers if the `E` flag in the status register is set.

```asm
jeq location: label
```
Jumps to *location* if the `E` flag in the status register is set.

```asm
jeq x: reg, y: reg|imm
```
Jumps to the location pointed to by the HL registers if *x* == *y*.

```asm
jeq x: reg, y: reg|imm, location: label
```
Jumps to *location* if *x* == *y*.

#### JZ Macro

```asm
jz condition: reg|imm, location: label
```
Jumps to *location* if *condition* is 0.

#### CALL Macro

```asm
call
```
Pushes the return address to the stack and jumps to the address pointed to by the HL registers.
Designed to be paired with the `ret` macro.

```asm
call location: label
```
Pushes the return address to the stack and jumps to *location*.
Designed to be paired with the `ret` macro.

#### RET Macro

```asm
ret
```
Jumps to the address stored at the *top* of the stack.

#### MV16 Macro

```asm
mv16 high: reg, low: reg, imm: imm
```

Moves a 16-bit immediate integer into the provided registers.

#### ADD16 Macro

```asm
add16 h0: reg, l0: reg, h1: reg|imm, l1: reg|imm
```
Adds two 16-bit integers.
*h0* and *l0* make up the high and low bytes of the first operand,
with *h1* and *l1* making up the high and low bytes of the second operand.

#### SUB16 Macro

```asm
sub16 h0: reg, l0: reg, h1: reg|imm, l1: reg|imm
```
Subtracts two 16-bit integers.
*h0* and *l0* make up the high and low bytes of the first operand,
with *h1* and *l1* making up the high and low bytes of the second operand.

#### INC Macro

```asm
inc reg: reg
```
Adds 1 to the value contained in *reg*, storing the result back in *reg*.

```asm
inc high: reg, low: reg
```
Adds 1 to the 16-bit value contained in *high* and *low*,
storing the result back in *high* and *low*.

#### DEC Macro

```asm
dec reg: reg
```
Subtracts 1 from the value contained in *reg*, storing the result back in *reg*.

```asm
dec high: reg, low: reg
```
Subtracts 1 from the 16-bit value contained in *high* and *low*,
storing the result back in *high* and *low*.

#### NOT Macro

```asm
not reg: reg
```
Performs a bitwise NOT operation on *reg*, storing the result back in *reg*.

#### AND Macro

```asm
and x: reg, y: reg|imm
```
Performs a bitwise AND operation on *x* and *y*, storing the result in *x*.

#### XOR Macro

```asm
and x: reg, y: reg|imm
```
Performs a bitwise XOR operation on *x* and *y*, storing the result in *x*.

#### SHL Macro

```asm
shl reg: reg
```
Performs a logical shift left on *reg*, storing the result back in *reg*

#### NOP Macro

```asm
nop
```
Performs an operation that has no effect, taking 4 clock cycles (the same as `ADD`).

#### USE Macro

```asm
use label: label|ident
```
Eliminates the `warning: unused label definition` message for *label*.

## Emulator

The f8ful emulator simulates each individual clock cycle,
using the CPU's microcode to determine what to do on each pulse.
An upside to this is that you can dump the CPU at any time and see the microcode and program counter for each clock pulse.
A downside, however, is that the emulator is much slower than it could be,
since it has to check every microcode flag for every clock pulse.

The emulator contains a REPL with a few useful commands:

* [SET](#set)
* [GET](#get)
* [PEEK](#peek)
* [POKE](#poke)
* [RUN](#RUN)
* [LOAD](#load)
* [DROP](#drop)
* [DUMP](#dump)
* [STEP](#step)
* [RESET](#reset)
* [STOP](#stop)
* [QUIT](#quit)
* [HELP](#help)

### SET

Syntax: `SET <register>, <value>`

Sets the value of *register* to *value*.

### GET

Syntax: `GET <register>`

Prints the data stored in *register*.

### PEEK

Syntax: `PEEK <address>`

Prints the data stored in memory at *address*.

### POKE

Syntax: `POKE <address>, <value>`

Sets the memory at *address* to *value*.

### RUN

Syntax: `RUN <speed>`

Runs the emulator clock at *speed* in HZ.
If *speed* is zero the emulator clock will run uncapped.

### LOAD

Syntax: `LOAD <path>, <address>`

Attaches the [peripheral](#peripherals) located at *path* to *address*.

### DROP

Syntax: `DROP <address>`

Drops the [peripheral](#peripherals) attached to *address* if there is one.

### DUMP

Syntax: `DUMP`

Prints the current machine state.
Includes information such as the program counter,
stack pointer, status register, ALU registers,
general purpose registers, etc...

### STEP

Syntax: `STEP`

Steps the emulator clock by one pulse.
Only works if the emulator clock is stopped.

### RESET

Syntax: `RESET`

Resets the CPU state to the initial state.

### STOP

Syntax: `STOP`

Stops the emulator clock.
Only works if the emulator clock is running.

### QUIT

Syntax: `QUIT`

Drops all peripherals and quits the emulator.

### HELP

Syntax: `HELP`

Prints a help message detailing the REPL's commands.

### MMIO

There are several locations in memory with mapped IO.
These memory-mapped addresses allow programs to interact with hardware directly.
The top 48 memory addresses are reserved for various peripherals,
with two implemented in the emulator.

 * `0xFFFF` is where the status register (SREG) resides.
 * `0xFFFE` is the low byte of the stack pointer.
 * `0xFFFD` is the high byte of the stack pointer.

Below these reserved addresses, the address range `0xF000` through `0xFFCF` are reserved for the video memory.
This address range is functionally similar to VGA text mode in x86 processors,
with an 85x20 character screen.

The low (character) byte is the code point.
The VGA text follows code-page 737 seen below:

![Code-page 737](./misc/characters.png)

The second byte is the attribute or modifier byte,
describing the foreground and background colors.
The lower nibble describes the foreground color,
while the upper nibble describes the background color.

![An example of all foreground and background colors](./misc/colors.png)
*<sub>An example of all foreground and background colors</sub>*

## Tests

Fateful has a built-in test suite that can make it easy to make sure
your program actually does what it is supposed to.
You can run tests on assembly programs with the `test` command:
```bash
fateful test <program>.asm
```

The test command will check the contents of registers after halting if specified in the program.
These checks are specified in rich comments (`///`) similar to libraries.
For example, these are the checks included in the `fib.asm` example:
```rust
/// a: 0x0D
/// b: 0x15
/// c: 0x00
/// d: 0x15
```

These are only read by the test suite, and will be checked after the emulator halts.
In this example, the test-runner will assert that the content of the A register is `0x0D`,
the B register is `0x15`, the C register is `0x00`, and the D register is `0x15`.
If these assertions fail, the test is marked as failing.

The test command also includes a `--timeout` flag, which defaults to `500ms`.
If the emulator does not detect a halt in this time,
the emulator will exit and the test will be marked as failing.

## Peripherals

Peripherals are a way to extend the emulator,
simulating a memory-mapped peripheral.
This is done through the use of dynamic library loading,
so you can create a peripheral in any language that supports the C ABI.
Peripherals can be attached to one or more slots in the top 48 bytes of RAM.

### Stateless Peripherals

Stateless peripherals are the simplest form of peripheral,
with state being managed by the peripheral rather than the emulator.

#### Initialization

Stateless peripherals are initialized by a function with the signiture `int init(unsigned char)`.
The return value of this is expalined in [errors](#errors).
The input parameter provides the number of slots that the peripheral has been attached to.

#### Reading and Writing

Peripherals can be written through a function with the signiture `void write(unsigned char, unsigned char)`.
This function is called whenever the CPU writes to the given address or the address is `POKE`ed.
The first parameter is the slot index that is being written to, and the second parameter is the value being written.

Peripherals can be read from with a function with the signiture `unsigned char read(unsigned char)`.
This function is called whenever the CPU reads from the given address or the address is `PEEK`ed at.
The input parameter is the slot index that is being read from, and the return value should be the value at the slot.

#### Drop

Peripherals are dropped through a function with the signiture `void drop()`.
This function is called when every address this peripheral is attached to is `DROP`ed, or the emulator is quit.
This function *must* clean up any seperate threads before returning or the emulator will crash.

#### Reset

Peripherals are reset through a function with the signiture `void reset()`.
This functions is called whenever the emulator resets the CPU.

### Stateful Peripherals

Stateful peripherals are a way to offload managing state to the emulator.
They essentially allow a peripheral to hand off a heap-allocated pointer to that peripheral's state.
This is especially important if you are spawning multiple threads and need to rejoin the main thread,
or if you want to have multiple instances of the same peripheral.

There is a Rust crate - [`fateful_peripheral`](https://github.com/commonkestrel/fateful_peripheral) -
that handles all of the pointer magic behind the scenes, allowing you to stay completely in the `safe` world.

#### Initialization

When a stateful peripheral is initialized through a function of the signiture `*void stateful_init(char)`,
it must return a pointer to the state's location.
**Warning:** This pointer *must* be stored on the heap,
otherwise the emulator *will* Segfault.
Other than the pointer return,
the rest of the function should be the same as its stateless counterpart.

If an error occurs during the setup,
the process for reporting these is explained in [Errors](#errors).

#### Reads, Writes, Drops, and Resets

Reading, writing, dropping, and resetting stateful peripherals is functionally the same as stateless,
but each function has an extra `stateless_` prepended to its identifier,
as well as a pointer parameter (`*void`) at the start of each functions's parameters.

### Errors

Errors are only checked upon initialization - after both `init` and `stateful_init`.
If either `init` returns a non-zero value or `stateful_init` returns a null pointer,
the emulator will check for a function with the signiture `int last_error_length()`.
If this function exists, the emulator will then check for a function with the signiture `*char last_error()`.
`last_error_length` should return the length of the ASCII string pointed to by the result of `last_error`.

### Names

Peripherals can optionally have a name that will displayed when the emulator is `DUMP`ed.
This must be supplied by a function with the following signiture: `*char name()`.
The returned pointer must point to a null-terminated ASCII string.
