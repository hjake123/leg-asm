# Towers of Hanoi! Why not...
# Uses r4 as a parameter stack index since you never implemented call stack push/pull opcodes.
# r0-3 are number, source, dest, spare respectively
const stack 0xF
const magnet 5

MOV io r0
MOV io r1
MOV io r2
MOV io r3
MOV stack r4
call move

label move
BN r0 0 disks_remain
CALL mdisk
RET
label disks_remain

# First recursive call
CALL push
SUB r0 1 r0
MOV 0 addr
SAVE r2
MOV r3 r2
LOAD r3
CALL move
CALL pop

# Move disk
CALL mdisk

# Second recursive call
CALL push
SUB r0 1 r0
MOV 0 addr
SAVE r1
MOV r3 r1
LOAD r3
CALL move
CALL pop
RET

label mdisk
# Move a disk from r1 to r2
MOV r1 io
MOV magnet io
MOV r2 io
MOV magnet io
RET

label push
MOV r4 addr
SAVE r0
ADD addr 1 addr
SAVE r1
ADD addr 1 addr
SAVE r2
ADD addr 1 addr
SAVE r3
ADD addr 1 r4
RET

label pop
SUB r4 4 r4
MOV r4 addr
LOAD r0
ADD addr 1 addr
LOAD r1
ADD addr 1 addr
LOAD r2
ADD addr 1 addr
LOAD r3
RET