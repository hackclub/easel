/// a: 0x20
/// b: 0x40

@dseg
@double VARIABLE

@cseg

mv E, 0x20
mv F,0x40
st [$VARIABLE], E
st [$VARIABLE + 1], F

ld A, [$VARIABLE]
ld B, [$VARIABLE + 1]

halt
