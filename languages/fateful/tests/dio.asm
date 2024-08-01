@define PULLA 0xFFFA
@define PUSHA 0xFFF9
@define DDRA  0xFFF8

@org 0x0000

_start:
    call [setup]
.loop:
    call [loop]
    jmp [.loop]

setup:
    mv C, 0b0000_0010
    st [DDRA], C
    mv C, 0xFF
    st [PUSHA], C
    ret

loop:
    ld A, [PULLA]
    and A, 0b0000_0001
    jnz A, [.off]
.on:
    mv B, 0b1111_1111
    jmp [.write]
.off:
    mv B, 0b1111_1101
.write:
    st [PUSHA], B
    ret