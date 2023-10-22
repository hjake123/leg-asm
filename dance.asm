MOV io r0
label loop
# temp1 = seed xor (seed shr 1)
RSHIFT r0 1 r1
XOR r0 r1 r0
# temp2 = temp1 xor (temp1 shl 1)
LSHIFT r0 1 r1
XOR r0 r1 r0
# next = temp2 xor (temp2 shr 2)
RSHIFT r0 2 r1
XOR r0 r1 r0
MOD r0 4 io
JUMP loop