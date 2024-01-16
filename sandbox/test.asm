MOV 1 r0
label start
ADD r0 io r1
SAVE r1
LOAD r0
MOV r0 io
BG r0 10 distant
JUMP start
label distant
HALT