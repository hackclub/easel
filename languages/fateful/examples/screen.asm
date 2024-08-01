@org 0x0000
@define BUF_START 0xF000
@define COLOR_START (BUF_START + 256*2) ; Start offset by 256 characters to not overwrite any of the char display

_start:
    mv A, 0
    mv B, 0x0F
.char_loop:
    lda [BUF_START]
    add16 H, L, 0x00, A
    add16 H, L, 0x00, A ; double offset to account for modifier bytes
    st A
    inc H, L ; increment to shift into modifier byte
    st B
    
    inc A
    jnz A, [.char_loop]

    mv A, 0
    mv B, 0x01 ; smiley face
.color_loop:
    lda [COLOR_START]
    add16 H, L, 0x00, A
    add16 H, L, 0x00, A ; double offset to account for modifier bytes
    st B
    inc H, L ; increment to shift into modifier byte
    st A

    inc A
    jnz A, [.color_loop]
    halt
