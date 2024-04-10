mod processor;
mod decoder;

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Machine {
    prom: [u8; 256],
    memory: [u8; 256],
    stack: Stack,
    registers: RegisterBank
}

#[derive(Debug)]
struct RegisterBank { reg: [u8; 8] }

const ADDR: u8 = 5;
const PC: u8 = 6;
const IO: u8 = 7;

impl RegisterBank {
    pub fn new() -> RegisterBank {
        RegisterBank { reg: [0; 8] }
    }
}

// Indexing the RegisterBank returns its registers!
impl Index<u8> for RegisterBank {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        let index: usize = index.into();
        &self.reg[index]
    }
}

impl IndexMut<u8> for RegisterBank {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        let index: usize = index.into();
        &mut self.reg[index]
    }
}

#[derive(Debug)]
pub struct Stack {
    stack: [u8; 256],
    stack_ptr: u8
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack: [0; 256], stack_ptr: 0 }
    }

    pub fn push(&mut self, val: u8) {
        let ptr: usize = self.stack_ptr.into();
        self.stack[ptr] = val;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> u8 {
        self.stack_ptr -= 1;
        let ptr: usize = self.stack_ptr.into();
        self.stack[ptr]
    }}

impl Machine {
    // Load a program into the machine's PROM
    pub fn load(program: &str) -> Machine{ 

        let mut prom_end = 0;
        let mut prom: [u8; 256] = [0; 256];

        for line in program.lines(){
            if prom_end > 255 {
                panic!("Program was too long to fit into prom!")
            }
            prom_end = parse_line(line, &mut prom, prom_end);
        }

        Machine { 
            prom, 
            memory: [0;256], 
            stack: Stack::new(), 
            registers: RegisterBank::new()
        }
    }

    // Simulate one cycle of execution. Returns true unless HALT runs.
    pub fn cycle(&mut self) -> bool {
        let pc: usize = self.registers[PC].into();
        let inst = decoder::Instruction {
            opcode: self.prom[pc],
            arg1: self.prom[pc + 1],
            arg2: self.prom[pc + 2],
            arg3: self.prom[pc + 3]
        };

        if inst.opcode == 255 {
            // HALT opcode
            return false
        }

        let flags = inst.decode();
        
        processor::execute(flags, self);

        self.registers[PC] = self.registers[PC].wrapping_add(4); // TODO: don't advance sometimes.
        
        true
    }

}

fn parse_line(line: &str, prom: &mut[u8;256], prom_end: usize) -> usize {
    let tokens: Vec<&str> = line.trim().split_whitespace().collect();
    if tokens.contains(&"#"){
        return prom_end;
    }

    prom[prom_end] = tokens[0].parse().expect("Invalid token");
    prom[prom_end + 1] = tokens[1].parse().expect("Invalid token");
    prom[prom_end + 2] = tokens[2].parse().expect("Invalid token");
    prom[prom_end + 3] = tokens[3].parse().expect("Invalid token");
    prom_end + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn bad_input(){
        let _ = Machine::load("fubar");
    }

    #[test]
    fn load_does_something(){
        let empty = Machine { 
            prom: [0;256], 
            memory: [0;256], 
            stack: Stack::new(), 
            registers: RegisterBank::new()
        };
        assert_ne!(Machine::load("64 7 0 0\n64 0 0 7\n32 0 0 0").prom, empty.prom);
    }

    #[test]
    fn load_works(){
        let real_prom: [u8; 12] = [64, 7, 0, 0, 64, 0, 0, 7, 32, 0, 0, 0];
        assert!(Machine::load("64 7 0 0\n64 0 0 7\n32 0 0 0").prom.starts_with(&real_prom));
    }

    #[test]
    fn parse_line_works(){
        let line = "64 32 0 128";
        let mut prom = [0;256];
        parse_line(line, &mut prom, 0);
        assert!(prom[0] == 64);
    }

    #[test]
    fn stack_test(){
        let mut stack = Stack::new();
        stack.push(20);
        stack.push(10);
        assert_eq!(stack.pop(), 10);
        assert_eq!(stack.pop(), 20);
    }
}
