# Sort the fifteen items and output them in order.

MOV 0 addr
label blankloop
SAVE 0xFF
ADD addr 1 addr
BL addr 15 blankloop

MOV 0 r4
label sortloop
MOV io r0
CALL insert
ADD r4 1 r4
BL r4 15 sortloop

MOV 0 addr
label echoloop
LOAD io
ADD addr 1 addr
BL addr 15 echoloop

label insert
# Function to insert r0 into the array sorted. 
MOV 0 addr
label scanloop
LOAD r1
BE r1 0xFF reached_scanloop_end
BL r0 r1 insert_at_addr
ADD addr 1 addr
JUMP scanloop

label insert_at_addr
MOV addr r3
label iaa_loop
ADD addr 1 addr
LOAD r2
SAVE r1
MOV r2 r1
BN r2 0 iaa_loop
MOV r3 addr
label reached_scanloop_end
SAVE r0
RET