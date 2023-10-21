'''
An assembler for my Turing Complete game's LEG architecture.
This was made because the in-game assembler is kinda annoying to work with.

Programs will be output in line broken decimal machine code.

Specification date: 10/21/2023
'''
import argparse

def find_labels_and_consts(lines: list) -> dict:
    '''
    Create a dictionary of label and const names to their byte position and value respectively.
    Assumes that the start of the program is at byte 0 and each instruction is 4 bytes.
    '''
    labels = {}
    byteindex = 0
    for line in lines:
        tokens = line.upper().split()
        if len(tokens) == 0 or tokens[0] == '#':
            continue
        if tokens[0] == 'LABEL':
            labels.update({tokens[1].upper(): byteindex})
            continue
        if tokens[0] == 'CONST':
            if tokens[2].isdigit():
                labels.update({tokens[1].upper(): int(tokens[2])})
            elif tokens[2][0] == "'":
                labels.update({tokens[1].upper(): ord(tokens[2][1])})
            else:
                print("Invalid const", tokens[2])
                exit(1)
            continue
        byteindex += 4
    return labels

def assemble_line(line: str, labels: dict) -> str:
    '''
    Assemble the line of decimal code needed to execute this line.
    Utilizes an already created list of label positions.
    '''
    opcode = 0
    arg0 = 0
    arg1 = 0
    arg2 = 0
    mode = 'invalid'
    tokens = line.upper().split()  
    if len(tokens) == 0:
        return ''
    if tokens[0][0] == '"':
        return line.strip()
    match tokens[0]:
        case 'ADD':
            # opcode = 0
            mode = 'alu'
            pass
        case 'SUB':
            opcode = 1
            mode = 'alu'
        case 'AND':
            opcode = 2
            mode = 'alu'
        case 'OR':
            opcode = 3
            mode = 'alu'
        case 'NOT':
            opcode = 4
            mode = 'alu'
        case 'XOR':
            opcode = 5
            mode = 'alu'
        case 'MULTH':
            opcode = 6
            mode = 'alu'
        case 'MULTL':
            opcode = 7
            mode = 'alu'
        case 'LSHIFT':
            opcode = 8
            mode = 'alu'
        case 'RSHIFT':
            opcode = 9
            mode = 'alu'
        case 'LROT':
            opcode = 10
            mode = 'alu'
        case 'RROT':
            opcode = 11
            mode = 'alu'
        case 'BE':
            opcode = 0b100000
            mode = 'compare'
        case 'BN':
            opcode = 0b100001
            mode = 'compare'
        case 'BL':
            opcode = 0b100010
            mode = 'compare'
        case 'BLE':
            opcode = 0b100011
            mode = 'compare'
        case 'BG':
            opcode = 0b100100
            mode = 'compare'
        case 'BGE':
            opcode = 0b100101
            mode = 'compare'
        case 'CALL':
            opcode = 0b100110
            mode = 'call'
        case 'JUMP':
            opcode = 0b100000
            mode = 'call'
        case 'RET':
            opcode = 0b100111
            mode = 'noargs'
        case 'SAVE':
            opcode = 0b10000
            mode = 'save'    
        case 'LOAD':
            opcode = 0b11000
            mode = 'load'   
        case 'MOV':
            opcode = 3
            mode = 'move'   
        case '#':
            return ""
        case 'LABEL':                
            return ""
        case 'CONST':                
            return ""
    # Now that we've matched the token to its basic instruction,
    # we can read future lines
    match mode:
        case 'noargs':
            pass
        case 'alu':
            arg0 = get_num_arg(tokens[1], labels)
            if is_imm(tokens[1]):
                opcode |= 128
            arg1 = get_num_arg(tokens[2], labels)
            if is_imm(tokens[2]):
                opcode |= 64
            arg2 = get_num_arg(tokens[3], labels)
        case 'compare':
            arg0 = get_num_arg(tokens[1], labels)
            if is_imm(tokens[1]):
                opcode |= 128
            arg1 = get_num_arg(tokens[2], labels)
            if is_imm(tokens[2]):
                opcode |= 64
            if not tokens[3] in labels:
                print("Invalid label", tokens[3])
                exit(1)
            arg2 = labels[tokens[3]]
        case 'call':
            if not tokens[1] in labels:
                print("Invalid label", tokens[1])
                exit(1)
            arg2 = labels[tokens[1]]
        case 'save':
            arg0 = get_num_arg(tokens[1], labels)
            if is_imm(tokens[1]):
                opcode |= 128
            opcode |= 64
        case 'load':
            arg2 = get_num_arg(tokens[1], labels)
        case 'move':
            arg0 = get_num_arg(tokens[1], labels)
            if is_imm(tokens[1]):
                opcode |= 128
            arg2 = get_num_arg(tokens[2], labels)
            opcode |= 64
        case default:
            print('Invalid instruction', tokens[0])
            exit(1)
    
    return str(opcode) + ' ' + str(arg0) + ' ' + str(arg1) + ' ' + str(arg2)
    
regs = {'R0': 0, 'R1': 1, 'R2': 2, 'R3': 3, 'R4': 4, 'R5': 5, 'ADDR': 5, 'PC': 6, 'IO': 7}

def is_imm(token) -> int:
    return not token in regs

def get_num_arg(token, labels) -> int:
    if token in regs:
        return regs[token]
    elif token in labels:
        return labels[token]
    elif token.isdigit():
        return token
    else:
        print('Invalid argument', token)
        exit(1)

parser = argparse.ArgumentParser("legasm.py")
parser.add_argument('in', help="LEG ASM file to be assembled")
parser.add_argument('-o', '--out', type=str, help="filename to be produced, defaults to 'leg.out'", default='leg.out')
parser.add_argument('-a', '--append', action='store_true', help="append the original program in comments")
args = vars(parser.parse_args())

with open(args['in']) as infile:
    with open(args['out'], mode='w') as outfile:
        print("# Assembled with legasm.py", file=outfile)
        labels = find_labels_and_consts(infile.readlines())
        infile.seek(0)
        for line in infile:
            code = assemble_line(line, labels)
            if not code == '':
                print(code, file=outfile)
        if args['append']:
            infile.seek(0)
            print(file=outfile)
            print("# Original:", file=outfile)
            for line in infile:
                if line.strip() == '':
                    continue
                print('#', line, end='', file=outfile)