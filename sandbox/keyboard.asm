const shift 21
const capslock 20
const escape 1
const backspace 4
const delete 8

label loop
MOV io r0
BE r0 0 loop
BE r0 shift loop
BE r0 capslock loop
BE r0 escape exit
BN r0 backspace not_backspace
CALL rmchar
JUMP loop
label not_backspace
BE r0 delete handle_delete
SAVE r0 
ADD addr 1 addr
JUMP loop
label exit
HALT

label rmchar
SUB addr 1 addr
SAVE 0
RET

label handle_delete
CALL rmchar
BG addr 0 handle_delete
JUMP loop