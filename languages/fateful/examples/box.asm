@define TEXT_BUFFER 0xF000
@define SCREEN_WIDTH 80
@define SCREEN_HEIGHT 25
@define BOX_WIDTH (SCREEN_WIDTH - 2)
@define BOX_HEIGHT (SCREEN_HEIGHT - 2)
@define HELLO_X 3
@define HELLO_Y 3

@define TR_CORNER 0xBB 
@define BR_CORNER 0xBC
@define BL_CORNER 0xC8
@define TL_CORNER 0xC9
@define WALL 0xBA
@define DASH 0xCD

@define STYLE 0x0F

/// math = https://github.com/commonkestrel/fateful_math
@include <math/mul.asm>

@cseg
@org 0x0000

_start:
    call [draw_top]
    call [draw_bottom]
    call [draw_left]
    call [draw_right]
    call [draw_hello]
    halt

draw_top:
    mv A, 1 ; A contains the X coordinate
    push A, 1, TL_CORNER ; x, y, character
    call [draw_character]
    mv B, (BOX_WIDTH - 2)
.loop:
    inc A
    push A, B

    push A, 1, DASH
    call [draw_character]

    pop B, A

    dec B
    jnz B, [.loop]

    inc A
    push A, 1, TR_CORNER
    call [draw_character]

    ret

draw_bottom:
    mv A, 1 ; A contains the X coordinate
    push A, BOX_HEIGHT, BL_CORNER ; x, y, character
    call [draw_character]
    mv B, (BOX_WIDTH - 2)
.loop:
    inc A
    push A, B

    push A, BOX_HEIGHT, DASH
    call [draw_character]

    pop B, A

    dec B
    jnz B, [.loop]

    inc A
    push A, BOX_HEIGHT, BR_CORNER
    call [draw_character]

    ret

draw_left:
inc A
    mv A, 1 ; A contains the Y coordinate
    mv B, (BOX_HEIGHT - 2)
.loop:
    inc A
    push A, B

    push 1, A, WALL
    call [draw_character]

    pop B, A

    dec B
    jnz B, [.loop]

    ret

draw_right:
    mv A, 1 ; A contains the Y coordinate
    mv B, (BOX_HEIGHT - 2)
.loop:
    inc A
    push A, B

    push BOX_WIDTH, A, WALL
    call [draw_character]

    pop B, A

    dec B
    jnz B, [.loop]

    ret

draw_hello:
    mv A, 0 ; character index stored in `A`
.loop:
    mv B, A
    add B, HELLO_X

    lda [hello_str]
    add16 H, L, 0, A
    lpm C
    jz C, [.return]

    push A ; store X coordinate
    push B, HELLO_Y, C ; pass X, Y, and character
    call [draw_character]

    pop A ; get X coordinate
    inc A

    jmp [.loop]
.return:
    ret

draw_character:
    pop H, L ; save return address
    pop C, B, A ; character, Y, X
    push L, H ; store return address
    push A, C; save X coordinate and character

    push B, 0, SCREEN_WIDTH, 0
    call [mul16] ; calculate Y offset
    pop H, L ; get value of Y offset

    pop C, A ; get character and X coordinate

    add16 H, L, 0, A ; add X to address
    add16 H, L, H, L ; double address to account for modifier bytes
    add16 H, L, (TEXT_BUFFER >> 8), (TEXT_BUFFER & 0xFF) ; Shift address to text-buffer space

    st C

    inc H, L
    mv C, STYLE
    st C

    ret

hello_str:
    @str "Hello world!"
