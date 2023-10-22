MOV io r0
MOV io r1
CALL divide
MOV r3 io
MOV r2 io
label end
JUMP end

label divide
# Divide r0 by r1
# Leave quotient in r3
# and remainder in r2
ADD 0 r0 r2
MOV 0 r3
label divloop
BL r2 r1 done_dividing
SUB r2 r1 r2
ADD 1 r3 r3
JUMP divloop
label done_dividing
RET