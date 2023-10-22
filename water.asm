# This is it--! Water world!
# First, read in the 16 height values.
MOV 15 r1
label readloop
MOV io r0
MOV r1 addr
SAVE r0
SUB r1 1 r1
BN r1 255 readloop

# Find the height occlusion from the left for each position.
const lheights 16
MOV lheights r1
label loccloop
SUB r1 lheights addr
LOAD r0
BLE r0 r2 locc_r0_cond_skip
MOV r0 r2
label locc_r0_cond_skip
MOV r1 addr
SAVE r2
ADD r1 1 r1
BN r1 32 loccloop

# Find the height occlusion from the right for each position.
const rheights 32
MOV 0 r2
ADD rheights 15 r1
label roccloop
SUB r1 rheights addr
LOAD r0
BLE r0 r2 rocc_r0_cond_skip
MOV r0 r2
label rocc_r0_cond_skip
MOV r1 addr
SAVE r2
SUB r1 1 r1
BN r1 31 roccloop

# Find sum of min(lheights[i], rheight[i]) - heights[i] for i in 1 to 14
# Let r1 be i, and r0 be the sum
MOV 1 r1
MOV 0 r0
label sumloop
ADD r1 lheights addr
LOAD r2
ADD r1 rheights addr
LOAD r3
MOV r1 addr
LOAD r4
BL r2 r3 lheights_lower
# rheights[i] was lower so use r3
SUB r3 r4 r4
ADD r0 r4 r0
JUMP sumloop_cond_end
label lheights_lower
# lheights[i] was lower so use r2
SUB r2 r4 r4
ADD r0 r4 r0
label sumloop_cond_end
ADD r1 1 r1
BLE r1 14 sumloop

MOV r0 io

label end
JUMP end