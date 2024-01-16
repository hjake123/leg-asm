MOV data addr
JUMP code
label data
[10, 20, 30, 40]
label code
PROM r0
SAVE r0
ADD addr 1 addr
BL addr code code
HALT