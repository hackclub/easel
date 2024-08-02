/// a: 127
/// c: 0xA1
/// d: 0x40


mv A, 120
mv B, 7

add A, B
sub B, A

mv16 C, D, 40_000
mv16 E, F, 1_280
add16 C, D, E, F

halt
