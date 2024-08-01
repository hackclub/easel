add:
    li
    rsb | rbo | aom | aol | pci
.reg:
    rsp | rbo | aoh
.imm:
    po | aoh
.both:
    ao | rsb | rbi | cr | pci

sub:
    li
    rsb | rbo | aom | aol | pci
.reg:
    rsp | rbo | aoh
.imm:
    po | aoh
.both:
    ao | aol | rsb | rbi | cr | pci

adc:
    li
    rsb | rbo | aom | aol | pci
.reg:
    rsp | rbo | aoh
.imm:
    po | aoh
.both:
    ao | aom | rsb | rbi | cr | pci

sbb:
    li
    rsb | rbo | aom | aol | pci
.reg:
    rsp | rbo | aoh
.imm:
    po | aoh
.both:
    ao | aol | aom | rsb | rbi | cr | pci

nand:
    li
    rsb | rbo | aom | aol | pci
.reg:
    rsp | rbo | aoh
.imm:
    po | aoh
.both:
    ao | aoh | rsb | rbi | cr | pci

or:
    li
    rsb | rbo | aom | aol | pci
.reg:
    rsp | rbo | aoh
.imm:
    po | aoh
.both:
    ao | aoh | rsb | rbi | cr | pci

cmp:
    li
    rsb | rbo | aom | aol | pci
.reg:
    rsp | rbo | aoh
.imm:
    po | aoh
.both:
    aol | cr | pci

mv:
    li
.reg:
    sr | pci
.imm:
    rsb | rbi | po | pci
.both:
    pci | cr

ld:
    li
.reg:
    thl | rbo
    la | rsb | rbi | cr | pci
.imm:
    ahi | po | pci
    ali | po | pci
    la | rsb | rbi | cr | pci

st:
    li
.reg:
    thl | rbo
    sa | rsb | rbo | cr | pci
.imm:
    ahi | po | pci
    ali | po | pci
    sa | rsb | rbo | cr | pci

lda:
.imm:
    li
    po | ahi | pci
    po | ali | pci
    thl | rbi | cr | pci

lpm:
    li
.reg:
    thl | rbo
    lpm | rsb | rbi | cr | pci
.imm:
    po | ahi | pci
    po | ali | pci
    lpm | rsb | rbi | cr | pci


push:
    li
.reg:
    lsp
    rsb | rbo | sa
.imm:
    lsp | pci
    po | sa
.both:
    spd | cr | pci

pop:
.reg:
    li
    spi
    lsp
    rsb | rbi | la | cr | pci

jnz:
    li
.reg:
    rsb | rbo | aom | aol
    aom | jnz | cr
.imm:
    po | aom | aol | pci
    aom | jnz | cr

halt:
    li
    sh
