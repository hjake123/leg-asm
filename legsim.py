'''
A simulator for my Turing Complete game's LEG (EX) architecture.
This consumes a file of line-broken integer 'machine code' and runs it as an executable.

Specification date: 12/24/2023

Details TBD
'''
import argparse
import keyboard

def matches(num: int, pattern: int) -> bool:
    '''
    Returns whether num completely fills a specified bit mask.
    '''
    return num & pattern == pattern

def read_register_or_io(index: int, registers: list, text_mode: bool, infile = None):
    '''
    Read from the register or query the user for input if it is register 7.
    If it returns None, the user has typed 'pause' and the program should pause.
    '''
    if index == 7:
        while True:
            if infile == None:
                print("INPUT :", end=" ")            
                i = input()
            else:
                i = infile.read(1)
                if len(i) == 0:
                    print('Finished consuming input file')
                    infile = None
                    return None
            if i == 'pause':
                return None
            if text_mode:
                if len(i) == 1:
                    return ord(i.encode('cp850')) 
            elif i.isnumeric():
                return int(i) & 0xFF
            
    return registers[index & 0b00000111]

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
        case 0b0100:
            return ~left
        case 0b0101:
            return left ^ right
        case 0b0110:
            return (left * right) & 0xFF00 >> 8
        case 0b0111:
            return (left * right) & 0xFF
        case 0b1000:
            return left << right
        case 0b1001:
            return left >> right
        case 0b1010:
            print('LROT NOT IMPLEMENTED')
            return left # TODO LROT
        case 0b1011:
            print('RROT NOT IMPLEMENTED')
            return left # TODO RROT
        case 0b1100:
            return left % right
        case 0b1101:
            return left / right
    return left

def run(prom: list, registers: list, ram: list, stack: list, simulator_args: dict, infile, outfile) -> bool:
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

    The returned bool is True unless execution should pause due to HALT or user input
    ''' 

    pc = registers[6]
    opcode = prom[pc]
    arg1 = prom[pc + 1]
    arg2 = prom[pc + 2]
    arg3 = prom[pc + 3]
    jumped = False

    # Check for HALT
    if opcode == 0b11111111:
        print("HALTED")
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
    elif prom_loading and len(prom) > registers[5]:
        left = prom[registers[5]]
    elif loading and len(ram) > registers[5]:
        left = ram[registers[5]]
    elif not loading and not prom_loading:
        left = read_register_or_io(arg1, registers, simulator_args['text_mode'], infile)
    
    if imm_right:
        right = arg2
    elif not loading and not prom_loading:
        right = read_register_or_io(arg2, registers, simulator_args['text_mode'], infile)

    if left == None or right == None:
        return False

    # Figure out the output bus.
    output = alu(left, right, opcode) & 0xFF
    out_blocked = matches(opcode, 0b00100000) or (matches(opcode, 0b00010000) and not matches(opcode, 0b00001000))
    if not out_blocked:
        if arg3 == 7:
            if simulator_args['text_mode']:
                encoded = bytes([output])
                print(encoded.decode('cp850'), end='', file=output_destination)
            else:
                print(output, file=output_destination)
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

def parse_line(line: str) -> list:
    '''
    Parse one line into a list of four bytes.
    '''
    bytelist = []
    for piece in line.split():
        if piece == '#':
            break
        bytelist.append(int(piece) & 0xFF)
    return bytelist

def print_ram(ram, text_mode):
    if text_mode:
        print("RAM:", end=' ')
        for char in ram:
            encoded = bytes([char])
            print(encoded.decode('cp850'), end='')
        print()
    else:
        print("RAM:", ram)

def print_state(prom, registers, ram, stack, text_mode):
    print("INST:", prom[registers[6]], prom[registers[6] + 1], prom[registers[6] + 2], prom[registers[6] + 3])
    print("REGS:", registers)
    print_ram(ram, text_mode)
    print("STACK:", stack)

def memedit(ram: list, text_mode: bool):
    '''
    Allow the user to edit RAM directly.
    '''
    while(True):
        print("Type 'done' to leave the memory editor")
        print_ram(ram, text_mode)
        print("Address to modify:", end = ' ')
        ui = input()
        if(ui == 'done'):
            return
        try:
            address = int(ui)
            print('New value:', end=' ')
            val = input()
            try:
                ram[address] = int(val) & 0xFF
            except:
                ram[address] = ord(val.encode('cp850')) 
        except:
            print()

    
parser = argparse.ArgumentParser("legsim.py")
parser.add_argument('program', help="LEG text file to be run")
parser.add_argument('-t', '--text_mode', action='store_true', help="Treat output and RAM as characters")
parser.add_argument('-i', '--input', type=str, help="get IO input from a file", default='')
parser.add_argument('-o', '--output', type=str, help="send IO output to a file", default='')
args = vars(parser.parse_args())
step_mode = True
print("Simulating", args['program'], end = ' ')
if(args['text_mode']):
    print('using text mode')
else:
    print('using int mode')
prom = []
with open(args['program']) as infile:
    for line in infile:
        prom.extend(parse_line(line)) 
registers = [0, 0, 0, 0, 0, 0, 0]
ram = []
stack = []
input_source = None
output_destination = None
if len(args['input']) > 0:
    input_source = open(args['input'])
    print('Opened input source', args['input'])
if len(args['output']) > 0:
    output_destination = open(args['output'], mode='w')
    print('Opened output destination', args['output'])

print("Type 'help' for commands")
while(True):
    if step_mode:
        print(">", end = ' ')
        command = input()
        if command == 'exit':
            break
        if command == 'run':
            print("Press ESC or type 'pause' at an input prompt to pause execution")
            step_mode = False
        if command == 'mem':
            memedit(ram, args['text_mode'])
        if command == 'view':
            print_state(prom, registers, ram, stack, args['text_mode'])
        if command == 'step':
            print_state(prom, registers, ram, stack, args['text_mode'])
            run(prom, registers, ram, stack, args, input_source, output_destination)
        if command == 'help':
            print("exit : quit the simulator")
            print("run : leave step mode")
            print("view : show the simulation state")
            print("step : perform one cycle of execution")
            print("mem : open the memory editor")
            print("help : show this list")

    elif keyboard.is_pressed('escape'):
        print("Type 'help' for commands")
        step_mode = True
    else:
        step_mode = not run(prom, registers, ram, stack, args, input_source, output_destination)