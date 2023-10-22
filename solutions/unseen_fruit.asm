const TRL 0
const FWD 1
const TRR 2
const WAIT 3
const USE 4
const SHT 5

const Conveyer 92

# First get into position.
mov TRR io
mov FWD io
mov TRR io
mov FWD io
mov FWD io
mov FWD io
mov FWD io
mov TRR io
mov FWD io
mov TRL io
mov FWD io

# Start recording and waiting.
label waitloop
mov WAIT io
mov io r0
be r0 Conveyer waitloop

# If it's not a conveyer belt, scan memory until a null to see if you've seen this before.
mov 0 addr
label scanloop
load r1
be r1 0 new_fruit
be r0 r1 repeated_fruit
add addr 1 addr
jump scanloop

# Save new fruits wherever the scan ended
label new_fruit
save r0
jump waitloop

# When a fruit repeats, we're done
label repeated_fruit
mov TRR io
mov USE io