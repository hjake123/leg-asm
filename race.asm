# Just hardcode it! 0 - r, 1 - d, 2 - l, 3 - u
mov instructions addr
label loop
prom io
add addr 1 addr
jump loop
label instructions
[3,0,1,0,0,3,2,3,0,3,2,2,1,2,3,3,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,0,0,3,2,3,3,0,1,0,3,0,1,1,2,1,0,1,1,2,3,2,2,1,0,1,2,1,0,0,3,0,1]