@include <arithmetic>
@include <flow>
@include <util>

/// Arguments: A, B
/// Return: A
mul:
    push B, C
    mv C, B
    mv B, A
    jz C, [.done]
.loop
    add A, B
    dec C
    jnz C, [.loop]
.done:
    pop C, B
    ret
