@define SCREEN [0xFFFC]
@define WRITE_CMD 0x10

mv A, 0 ; 4 clocks
mv B, WRITE_CMD ; 4 clocks
mv C, 0 ; 4 clocks
mv D, 0xFF ; 4 clocks
.loop:
    lda SCREEN ; 5 clocks
    st B ; 3 clocks
    st C ; 3 clocks
    st A ; 3 clocks
    st D ; 3 clocks
    inc A ; 4 clocks
    jnz A, [.loop] ; 5 clocks + 3 clocks
halt ; 2 clocks
