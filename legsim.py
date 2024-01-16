'''
A simulator for my Turing Complete game's LEG (EX) architecture.
This consumes a file of line-broken integer 'machine code' and runs it as an executable.

Specification date: 12/24/2023

Details TBD
'''
import argparse

def matches(num: int, mask: int) -> bool:
    '''
    Returns whether num completely fills a specified bit mask.
    '''
    return num & mask == mask

def read_register_or_io(index: int, registers: list) -> int:
    '''
    Read from the register or query the user for input if it is register 7.
    '''
    if index == 7:
        while True:
            print("PROGRAM NEEDS INPUT BYTE:", end=" ")
            i = input()
            if i.isnumeric():
                return int(i)
            if len(i) == 1:
                return ord(i)

    return registers[index & 0b00000111]

def run(prom: list, registers: list, ram: list, stack: list) -> bool:
    '''
    Simulate one cycle of execution.

    'prom' is the program rom, taken from the input file. 
    It is where instructions are fetched from, or can be read with the PROM instructuion.

    'registers' should be a list of exactly seven integers, each representing one register.

    Indecies 5, 6, and 7* have special properties:
        5 is used to address RAM and PROM
        6 is the PC
        7 is mapped to IO 
        *(register-mapped io doesn't involve this list, so this index is outside its bounds intentionally)
    
    'ram' is a list of byte objects manipulated by LOAD and SAVE instructions.

    'stack' is the call stack, used by CALL and RET.

    The returned bool is True unless execution halted due to the HALT instruction.
    ''' 

    pc = registers[6]
    opcode = prom[pc]
    arg1 = prom[pc + 1]
    arg2 = prom[pc + 2]
    arg3 = prom[pc + 3]
    jumped = False

    # Check for HALT
    if opcode == 0b11111111:
        return False

    # Define the left, right, and output busses.
    left = 0
    right = 0
    output = 0

    # Determine what is being emitted onto left and right.
    imm_left = matches(opcode, 0b10000000)
    imm_right = matches(opcode, 0b01000000)
    loading = matches(opcode, 0b00011000)
    prom_loading = matches(opcode, 0b00011001)

    if imm_left:
        left = arg1
    elif prom_loading:
        left = prom[registers[5]]
    elif not loading:
        left = read_register_or_io(arg1, registers)
    elif len(ram) > registers[5]:
        left = ram[registers[5]]

    if imm_right:
        right = arg2
    elif not loading and not prom_loading:
        right = read_register_or_io(arg2, registers)

    # Figure out the output bus.
    output = alu(left, right, opcode)
    out_blocked = matches(opcode, 0b00100000) or (matches(opcode, 0b00010000) and not matches(opcode, 0b00001000))
    if not out_blocked:
        if arg3 == 7:
            print(output)
        else:
            registers[arg3] = output
        if arg3 == 6:
            jumped = True

    # SAVE
    if matches(opcode, 0b00010000):
        address = registers[5]
        while(len(ram) <= address):
            ram.append(0)
        ram[address] = output

    # Deal with flow control.
    condition = matches(opcode, 0b00100000) and left == right 
    condition = condition or matches(opcode, 0b00100001) and left != right 
    condition = condition or matches(opcode, 0b00100010) and left < right
    condition = condition or matches(opcode, 0b00100011) and left <= right
    condition = condition or matches(opcode, 0b00100100) and left > right
    condition = condition or matches(opcode, 0b00100101) and left >= right
    if condition:
        jumped = True
        registers[6] = arg3

    # Deal with functions
    if matches(opcode, 0b00100110):
        # We're calling!
        jumped = True
        stack.append(registers[6])
        registers[6] = arg3

    if matches(opcode, 0b00100110):
        # We're returning!
        jumped = True
        registers[6] = stack.pop()

    # Increment the PC and continue if we didn't just jump (or MOVE -> pc)
    if not jumped:
        registers[6] += 4
    return True

def alu(left: int, right: int, opcode: int) -> int:
    '''
    Perform the ALU computations.
    What happens is determined by the last four bits of opcode:
    0000 - OR | 0001 - AND | 0010 - ADD | 0011 - SUB
    0100 - NOT | 0101 - XOR | 0110 - MULT_HIGH | 0111 - MULT_LOW
    1000 - LSHIFT | 1001 - RSHIFT | 1010 - LROT | 1011 - RROT
    1100 - MOD | 1101 - DIV
    '''
    # TODO the rest of the math
    match (opcode & 0b00001111):
        case 0b0000:
            return left | right
        case 0b0001:
            return left & right
        case 0b0010:
            return left + right
        case 0b0011:
            return left - right
    return left

def parse_line(line: str) -> list:
    '''
    Parse one line into a list of four bytes.
    '''
    pieces = line.split()
    bytelist = []
    for piece_index in range(4):
        if pieces[piece_index][0] == '#':
            break
        bytelist.append(int(pieces[piece_index]))
    return bytelist

parser = argparse.ArgumentParser("legsim.py")
parser.add_argument('in', help="LEG text file to be run")
parser.add_argument('-s', '--step', action='store_true', help="Pause between steps.")
args = vars(parser.parse_args())

with open(args['in']) as infile:
    print("Simulating", args['in'])
    prom = []
    for line in infile:
        prom.extend(parse_line(line))
    registers = [0, 0, 0, 0, 0, 0, 0, 0]
    ram = []
    stack = []
    if args['step']:
        print("INST:", prom[registers[6]], prom[registers[6] + 1], prom[registers[6] + 2], prom[registers[6] + 3])
    while(run(prom, registers, ram, stack)):
        if args['step']:
            print("REGS:", registers, ">")
            input()
            print("INST:", prom[registers[6]], prom[registers[6] + 1], prom[registers[6] + 2], prom[registers[6] + 3])