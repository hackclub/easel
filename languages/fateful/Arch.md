# Architecture

The F8ful CPU is a custom CPU designed to run at 1 MHz,
though it can theoretically run up to 2.5 MHz.
It is inspired by Ben Eater's [8-bit computer](https://eater.net/8bit) and the [JDH-8](https://github.com/jdah/jdh-8).

## Control Word

The control word contains flags that control operations inside the CPU,
and enables CPU control based on instructions.

This control word changes every clock cycle,
and is determined in 3 ROM chips inside the control circuit
based on the current instruction, the instruction immediate bit,
and the clock cycle.

The control word is split into 3 bytes,
just because I couldn't find any EEPROMs with a 24-bit word.
It is layed out like so:

|           | Bit 7 | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0 |
|-----------|-------|-------|-------|-------|-------|-------|-------|-------|
| Low Byte  | `RSP` | `RSB` | `RBO` | `RBI` |  `AO` | `AOH` | `AOM` | `AOL` |
| Mid Byte  |  `SR` |  `PO` |  `LI` | `JNZ` | `PCI` |  `CR` | `SPD` | `SPI` |
| High Byte |  `SH` | `LPM` | `LSP` | `AHI` | `ALI` |  `SA` |  `LA` | `THL` |

### ALU Opcode

The first four bits of the control word (`AOL`, `AOM`, and `AOH`) represent the ALU opcode.
ALU operations based on these opcodes are shown below:

|  `AO` | `AOH` | `AOM` | `AOL` | Operation |
|-------|-------|-------|-------|-----------|
|  `0`  |  `0`  |  `0`  |  `0`  |   `NOP`   |
|  `0`  |  `0`  |  `0`  |  `1`  |   `CMP`   |
|  `0`  |  `0`  |  `1`  |  `0`  |    `CZ`   |
|  `0`  |  `0`  |  `1`  |  `1`  |   `ALP`   |
|  `0`  |  `1`  |  `0`  |  `0`  |   `ALS`   |
|  `1`  |  `0`  |  `0`  |  `0`  |   `ADD`   |
|  `1`  |  `0`  |  `0`  |  `1`  |   `SUB`   |
|  `1`  |  `0`  |  `1`  |  `0`  |   `ADC`   |
|  `1`  |  `0`  |  `1`  |  `1`  |   `SBB`   |
|  `1`  |  `1`  |  `0`  |  `0`  |  `NAND`   |
|  `1`  |  `1`  |  `0`  |  `1`  |    `OR`   |

The `AO` bit, or Arithmetic Operation bit, designates an arithmetic operation.
If `AO` is set, the output of the operation will be output to the bus.
When not set, the operation executed is a special operation,
or an operation that is not part of general integer arithmatic.
These need a bit more explanation.

- `NOP` (No Op) does nothing.
- `CMP` (Compare) compares the integers in the primary and secondary register,\
  setting the `L`, `E`, and `G` bits in the Status Register respectively.
- `CZ` (Check Zero) checks if the number in the primary register is `0`,\
  setting the `Z` bit in the Status Register respectively.
- `ALP` (ALU Load Primary) loads the ALU primary register from the bus.
- `ALS` (ALU Load Secondary) loads the ALU secondary register from the bus.

### Control Flags

The rest of the flags control individual operations in the CPU.
These operations are listed here:

- `RBI` (Register Bank In): Loads data from the bus into the current selected register.
- `RBO` (Register Bank Out): Outputs contents of the current selected register to the bus.
- `RSB` (Register Select Built-in): Selects the register indexed in the loaded instruction.
- `RSP` (Register Select Primary): Selects the register indexed in the current program byte.
- `SPI` (Stack Pointer Increment): Increments the Stack Pointer.
- `SPD` (Stack Pointer Decrement): Increments the Stack Pointer.
- `CR` (Clock Reset): Resets the clock in the control segment. This will always load the next instruction.
- `PCI` (Program Counter Increment): Increments the Program Counter.
- `JNZ` (Jump if Not Zero): Sets the program counter to the address in the H and L registers if the `Z` flag in the Status Register is set, otherwise increments the program counter.
- `LI` (Load Instruction): Loads the the current program byte into the instruction register.
- `PO` (Program Out): Outputs the current program byte onto the bus.
- `SR` (Store Register): Stores the register indexed in the instruction register into the register indexed in the current program byte.
- `THL` (Transfer HL): Links the bytes in the HL registers and the Address Register.
- `LA` (Load Address): Outputs the byte of RAM addressed in the Address Register to the bus.
- `SA` (Store Address): Stores the contents of the bus at the RAM location addressed in the Address Register.
- `ALI` (Address Low In): Loads the contents of the bus into the low byte of the Address Register.
- `AHI` (Address High In): Loads the contents of the bus into the high byte of the Address Register.
- `LSP` (Load Stack Pointer): Loads the Stack Pointer into the Address Register.
- `LPM` (Load Program Memory): Loads the byte indexed by the Address Register onto the bus.
- `SH` (Set Halt): Sets the Halt (`H`) bit in the Status Register.

## Memory

There are 64kb of accessable RAM on the board,
with the top 32 addresses (`0xFFE0` - `0xFFFF`) being reserved for memory mapped peripherals.
The stack starts at `0xFFDF` and grows downward.

There are 64 addresses for memory mapped I/O to allow for expansion,
but there are a few peripherals that are required:

- `0xFFFF` Status Register: This register is written to and read by the CPU without addressing,\
  but programs can access it through memory operations.\
  The contents of the Status Register are explained more in [Status Register](#status-register).
- `0xFFFE` Stack Pointer High: The top 8 bits of the stack pointer.
- `0xFFFD` Stack Pointer Low: The lower 8 bits of the stack pointer.

## Status Register

| $7$ | $6$ | $5$ | $4$ | $3$ | $2$ | $1$ | $0$ |
|-----|-----|-----|-----|-----|-----|-----|-----|
| `H` |     |     | `G` | `E` | `L` | `C` | `Z` |

- `H` (Halt)
- `G` (Greater Than)
- `E` (Equal)
- `L` (Less Than)
- `C` (Carry)
- `Z` (Zero)
