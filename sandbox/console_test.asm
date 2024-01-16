MOV data addr
MOV data r4
label loop
PROM r0
SAVE r0
ADD addr 1 addr
JUMP loop
label data
"Hello world!"