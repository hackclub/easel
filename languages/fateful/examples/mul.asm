/// Multiplies 5 and 120, storing the result in the H and L registers.
///
/// h: 0x02
/// l: 0x58

/// math = https://github.com/commonkestrel/fateful_math
@include <math/mul.asm>

@cseg
@org 0x0000
_start:
    push 5, 0, 120, 0
    call [mul16]
    pop H, L
    halt