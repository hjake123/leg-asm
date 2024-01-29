mov 1 r0
mov 1 r1
label loop
add r0 r1 r0
call swap
mov r0 io
bl r0 200 loop
halt

label swap
mov r0 r2
mov r1 r0
mov r2 r1
ret