// All pre-built macros

@macro push {
    ; push two values
    (%r0:reg|imm, %r1:reg|imm) {
        push %r0
        push %r1
    }
    ; push three values
    (%r0:reg|imm, %r1:reg|imm, %r2:reg|imm) {
        push %r0
        push %r1, %r2
    }
    ; push four values
    (%r0:reg|imm, %r1:reg|imm, %r2:reg|imm, %r3:reg|imm) {
        push %r0
        push %r1, %r2, %r3
    }
    ; push five values
    (%r0:reg|imm, %r1:reg|imm, %r2:reg|imm, %r3:reg|imm, %r4:reg|imm) {
        push %r0
        push %r1, %r2, %r3, %r4
    }
    ; push six values
    (%r0:reg|imm, %r1:reg|imm, %r2:reg|imm, %r3:reg|imm, %r4:reg|imm, %r5:reg|imm) {
        push %r0
        push %r1, %r2, %r3, %r4, %r5
    }
}

@macro pop {
    ; pop two registers
    (%r0:reg, %r1:reg) {
        pop %r0
        pop %r1
    }
    ; pop three registers
    (%r0:reg, %r1:reg, %r2:reg) {
        pop %r0
        pop %r1, %r2
    }
    ; pop four registers
    (%r0:reg, %r1:reg, %r2:reg, %r3:reg) {
        pop %r0
        pop %r1, %r2, %r3
    }
    ; pop five registers
    (%r0:reg, %r1:reg, %r2:reg, %r3:reg, %r4:reg) {
        pop %r0
        pop %r1, %r2, %r3, %r4
    }
    ; pop six registers
    (%r0:reg, %r1:reg, %r2:reg, %r3:reg, %r4:reg, %r5:reg) {
        pop %r0
        pop %r1, %r2, %r3, %r4, %r5
    }
}

@macro pusha () {
    push A, B, C, D, E, F
}

@macro popa () {
    pop F, E, D, C, B, A
}

@macro jmp {
    () {
        jnz 1
    }
    (%location:label) {
        lda %location
        jmp
    }
}

@macro jnz (%condition:reg|imm, %location:label) {
    lda %location
    jnz %condition
}

/// jump if less than
@macro jlt {
    (%x:reg, %y:reg|imm) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, (1 << 2)
        nand F, F
        jnz F
    }
    (%x:reg, %y:reg|imm, %location:label) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, (1 << 2)
        nand F, F
        jnz F, %location
    }
    (%location:label) {
        ld F, [0xFFFF]
        nand F, (1 << 2)
        nand F, F
        jnz F, %location
    }
    () {
        ld F, [0xFFFF]
        nand F, (1 << 2)
        nand F, F
        jnz F
    }
}

/// jump if less than or equal to
@macro jle {
    (%x:reg, %y:reg|imm) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, ((1 << 2) | (1 << 3))
        nand F, F
        jnz F
    }
    (%x:reg, %y:reg|imm, %location:label) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, ((1 << 2) | (1 << 3))
        nand F, F
        jnz F, %location
    }
    (%location:label) {
        ld F, [0xFFFF]
        nand F, ((1 << 2) | (1 << 3))
        nand F, F
        jnz F, %location
    }
    () {
        ld F, [0xFFFF]
        nand F, ((1 << 2) | (1 << 3))
        nand F, F
        jnz F
    }
}

/// jump if greater than
@macro jgt {
    (%x:reg, %y:reg|imm) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, (1 << 4)
        nand F, F
        jnz F
    }
    (%x:reg, %y:reg|imm, %location:label) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, (1 << 4)
        nand F, F
        jnz F, %location
    }
    (%location:label) {
        ld F, [0xFFFF]
        nand F, (1 << 4)
        nand F, F
        jnz F, %location
    }
    () {
        ld F, [0xFFFF]
        nand F, (1 << 4)
        nand F, F
        jnz F
    }
}

/// jump if greater than or equal to
@macro jge {
    (%x:reg, %y:reg|imm) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, ((1 << 4) | (1 << 3))
        nand F, F
        jnz F
    }
    (%x:reg, %y:reg|imm, %location:label) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, ((1 << 4) | (1 << 3))
        nand F, F
        jnz F, %location
    }
    (%location:label) {
        ld F, [0xFFFF]
        nand F, ((1 << 4) | (1 << 3))
        nand F, F
        jnz F, %location
    }
    () {
        ld F, [0xFFFF]
        nand F, ((1 << 4) | (1 << 3))
        nand F, F
        jnz F
    }
}

/// jump if equal
@macro jeq {
    (%x:reg, %y:reg|imm) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, (1 << 3)
        nand F, F
        jnz F
    }
    (%x:reg, %y:reg|imm, %location:label) {
        cmp %x, %y
        ld F, [0xFFFF]
        nand F, (1 << 3)
        nand F, F
        jnz F, %location
    }
    (%location:label) {
        ld F, [0xFFFF]
        nand F, (1 << 3)
        nand F, F
        jnz F, %location
    }
    () {
        ld F, [0xFFFF]
        nand F, (1 << 3)
        nand F, F
        jnz F
    }
}

@macro jz (%condition:reg|imm, %location:label) {
    jeq %condition, 0, %location
}

@macro call {
    () {
        push (($ + 6) & 0xFF) ; 2 bytes
        push (($ + 4) >> 8)   ; 2 bytes
        jmp                   ; 2 bytes
    }
    (%location:label) {
        push (($ + 9) & 0xFF) ; 2 bytes
        push (($ + 7) >> 8)   ; 2 bytes
        jmp %location         ; 5 bytes
    }
}

@macro ret () {
    pop H
    pop L
    jmp
}

/// Moves a 16 bit immediate into the provided registers
@macro mv16 (%high:reg, %low:reg, %imm:imm) {
    mv %high, (%imm >> 8)
    mv %low, (%imm & 0xFF)
}

/// Adds two 16-bit integers
@macro add16 (%h0:reg, %l0:reg, %h1:reg|imm, %l1:reg|imm) {
    add %l0, %l1
    adc %h0, %h1
}

/// Subtracts two 16-bit integers
@macro sub16 (%h0:reg, %l0:reg, %h1:reg|imm, %l1:reg|imm) {
    sub %l0, %l1
    sbb %h0, %h1
}

/// Increments the given value
@macro inc {
    ; 8-bit
    (%reg:reg) {
        add %reg, 1
    }
    ; 16-bit
    (%high:reg, %low:reg) {
        add %low, 1
        adc %high, 0
    }
}

/// Decrements the given value
@macro dec {
    ; 8-bit
    (%reg:reg) {
        sub %reg, 1
    }
    ; 16-bit
    (%high:reg, %low:reg) {
        sub %low, 1
        sbb %high, 0
    }
}

/// Bitwise inverts the given byte
@macro not (%reg:reg) {
    nand %reg, %reg
}

/// Bitwise and
@macro and (%x:reg, %y:reg|imm) {
    nand %x, %y
    nand %x, %x
}

/// Bitwise xor
@macro xor (%x:reg, %y:reg|imm) {
    mw F, %y
    or F, %x
    nand %x, %y
    and %x, F
}

@macro nop () {
    mv A, A ; shortest instruction with no side effects: 3 clock cycles
}

@macro use(%label:ident|label) {}

@macro shl (%r:reg) {
    add %r, %r
}
