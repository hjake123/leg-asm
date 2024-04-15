use std::io::{self, BufRead, Write};

use crate::{decoder::InstFlags, Machine, IO};

use self::branch::try_branch;

pub fn execute(flags: &InstFlags, machine: &mut Machine) {
    if flags.ret {
        fun::ret(machine);
        return
    }

    let left = if flags.left.1 { flags.left.0  } else if flags.left.0 == IO { get_user_input() } else { machine.registers[flags.left.0] };
    let right = if flags.right.1 { flags.right.0  } else if flags.left.0 == IO { get_user_input() } else { machine.registers[flags.right.0] };

    let output = 
        if flags.prom_loading {
            Some(mem::prom_load(machine))
        } else if flags.ram_loading {
            Some(mem::load(machine))
        } else {
            alu::execute(&flags.alu_op, left, right)
        };

    match flags.dest {
        Some(dest) => {
            // Dest is present for branches, so try to take a branch to dest.
            try_branch(&flags.cond, left, right, dest, machine);

            // Dest is present for function calls, so check for those.
            if flags.call {
                fun::call(dest, machine);
            }

            // If we have an output, save it to mem[addr] or register[dest].
            match output {
                Some(val) => {
                    if flags.save {
                        mem::save(val, machine);
                    } else if dest == IO {
                        print!("{} ", val);
                    } else {
                        machine.registers[dest] = val;
                    }
                },
                None => ()
            }
        },
        None => ()
    }
}

fn get_user_input() -> u8 {
    print!("\n> ");
    let _ = io::stdout().flush();
    let result = io::stdin().lock().lines().next();
    match result {
        Some(Ok(val)) => {
            match val.parse() {
                Ok(parsed_val) => parsed_val,
                Err(_) => { eprintln!("{val} is not a byte! Using 0"); 0 },
            }
        },
        Some(Err(err)) => { eprintln!("Tried to read standard input but {err}"); 0 },
        None => { eprintln!("Tried to read standard input but it was empty!"); 0 },
    }
}

mod alu {
    use crate::decoder::AluOperation;
    use std::ops::Not;

    pub fn execute(op: &Option<AluOperation>, left: u8, right: u8) -> Option<u8> {
        match op{
            Some(operation) => Some(match operation {
                AluOperation::Or => left | right,
                AluOperation::And => left & right,
                AluOperation::Add => left.wrapping_add(right),
                AluOperation::Sub => left - right,
                AluOperation::Not => left.not(),
                AluOperation::Xor => left ^ right,
                AluOperation::LeftShift => left << right,
                AluOperation::RightShift => left >> right,
                AluOperation::Mod => left % right,
                AluOperation::Div => left / right
            }),
            None => None
        }
    }
}

mod branch {
    use crate::{decoder::Condition, Machine, PC};

    pub fn try_branch(maybe_cond: &Option<Condition>, left: u8, right: u8, dest: u8, machine: &mut Machine) {
        match maybe_cond{
            Some(cond) => {
                let cond_met = compare(&cond, left, right);
                if cond_met {
                    machine.registers[PC] = dest;
                } else {
                    // Since this is a potentially jumping instruction, we have to advance manually...
                    machine.registers[PC] += 4; 
                }
            },
            None => { return; }
        }
    }

    fn compare(cond: &Condition, left: u8, right: u8) -> bool {
        match cond {
            Condition::Equal => left == right,
            Condition::NotEqual => left != right,
            Condition::Less => left < right,
            Condition::Greater => left > right,
            Condition::LessEqual => left <= right,
            Condition::GreaterEqual => left >= right
        }
    }
}

mod mem {
    use crate::{Machine, ADDR};

    pub fn prom_load(machine: &Machine) -> u8{
        machine.prom[address(machine)]
    }

    pub fn load(machine: &Machine) -> u8{
        machine.memory[address(machine)]
    }

    pub fn save(val:u8, machine: &mut Machine) {
        machine.memory[address(machine)] = val;
    }

    fn address(machine: &Machine) -> usize {
        machine.registers[ADDR].into()
    }
}

mod fun {
    use crate::{Machine, PC};

    pub fn call(addr: u8, machine: &mut Machine) {
        machine.stack.push(machine.registers[PC]);
        machine.registers[PC] = addr;
    }

    pub fn ret(machine: &mut Machine) {
        machine.registers[PC] =  machine.stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::{decoder::AluOperation, ADDR, PC};

    use super::*;

    #[test]
    fn add_exe() {
        assert_eq!(alu::execute(&Some(AluOperation::Add), 2, 2), Some(4));
    }

    #[test]
    fn branch_exe() {
        let mut machine = Machine::load("0 0 0 0");
        machine.registers[PC] = 4;
        try_branch(&Some(crate::decoder::Condition::Equal), 1, 1, 0, &mut machine);
        assert_eq!(machine.registers[PC], 0);
    }

    #[test]
    fn dont_branch_exe() {
        let mut machine = Machine::load("0 0 0 0");
        machine.registers[PC] = 4;
        try_branch(&Some(crate::decoder::Condition::NotEqual), 1, 1, 0, &mut machine);
        assert_ne!(machine.registers[PC], 0);
    }

    #[test]
    fn prom_exe() {
        let mut machine = Machine::load("41 42 43 44");
        machine.registers[ADDR] = 1;
        assert_eq!(mem::prom_load(&machine), 42);
    }

    #[test]
    fn save_load_exe() {
        let mut machine = Machine::load("0 0 0 0");
        machine.registers[ADDR] = 1;
        mem::save(42, &mut machine);
        assert_eq!(mem::load(&machine), 42);
    }

    #[test]
    fn imm_mov_exe() {
        let mut machine = Machine::load("0 0 0 0");
        let flags = InstFlags::new();
        let flags = InstFlags {
            alu_op: Some(AluOperation::Or),
            left: (1, true),
            dest: Some(0),
            ..flags
        };
        execute(&flags, &mut machine);
        assert_eq!(machine.registers[0], 1);
    }

    #[test]
    fn imm_mov_another_dest_exe() {
        let mut machine = Machine::load("0 0 0 0");
        let flags = InstFlags::new();
        let flags = InstFlags {
            alu_op: Some(AluOperation::Or),
            left: (1, true),
            dest: Some(4),
            ..flags
        };
        execute(&flags, &mut machine);
        assert_eq!(machine.registers[4], 1);
    }

    #[test]
    fn call_exe() {
        let mut machine = Machine::load("0 0 0 0");
        let flags = InstFlags::new();
        let flags = InstFlags {
            call: true,
            dest: Some(42),
            ..flags
        };
        execute(&flags, &mut machine);
        assert_eq!(machine.registers[PC], 42);
    }

    #[test]
    fn ret_exe() {
        let mut machine = Machine::load("0 0 0 0");
        machine.registers[PC] = 32;
        let flags = InstFlags::new();
        let flags = InstFlags {
            call: true,
            jumped: true,
            dest: Some(64),
            ..flags
        };
        execute(&flags, &mut machine);
        assert_eq!(machine.registers[PC], 64);
        let flags = InstFlags::new();
        let flags = InstFlags {
            ret: true,
            jumped: false,
            ..flags
        };
        execute(&flags, &mut machine);
        assert_eq!(machine.registers[PC], 32);
    }
}