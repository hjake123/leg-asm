mod processor;
mod decoder;

#[derive(Debug)]
struct RegisterBank(u8, u8, u8, u8, u8, u8, u8, u8);

impl RegisterBank{
    pub fn new() -> RegisterBank{
        RegisterBank(0, 0, 0, 0, 0, 0, 0, 0)
    }
}

#[derive(Debug)]
pub struct Machine {
    prom: [u8; 256],
    memory: [u8; 256],
    stack: [u8; 256],
    registers: RegisterBank
}

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
            stack: [0;256], 
            registers: RegisterBank::new()
        }
    }

    // Simulate one cycle of execution. Returns true unless HALT runs.
    pub fn cycle(&mut self) -> bool {
        let pc: usize = self.registers.6.into();
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
        println!("{:?}", flags);
        
        self.registers.6 = self.registers.6.wrapping_add(4);
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
            stack: [0;256], 
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
}
