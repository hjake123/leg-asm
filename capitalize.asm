const upper_factor 32
const space 32

label loop
MOV io r0
BE r1 0 capitalize
label after_if
SUB r0 space r1
MOV r0 io
JUMP loop

label capitalize
SUB r0 upper_factor r0
jump after_if