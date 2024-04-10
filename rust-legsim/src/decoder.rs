#[derive(Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub arg1: u8,
    pub arg2: u8,
    pub arg3: u8
}

impl Instruction {
    pub fn decode(&self) -> InstFlags {
        let mut left_reg = None;
        let mut right_reg = None;

        if matches(0b1000_0000, self.opcode) {
            left_reg = None;
        } else {
            left_reg = Some(self.arg1);
            
        }

        if matches(0b0100_0000, self.opcode) {
            right_reg = None;
        } else {
            right_reg = Some(self.arg2);
        }
        let dest = Some(self.arg3);

        let prom_loading = operation_matches(0b0001_1001, self.opcode);
        let ram_loading = operation_matches(0b0001_1000, self.opcode);
        let is_save = operation_matches(0b0001_0000, self.opcode);
        let call = operation_matches(0b0010_0110, self.opcode);
        let ret = operation_matches(0b0010_0111, self.opcode);

        let cond = 
            if operation_matches(0b0010_0000, self.opcode) { Some(Condition::Equal) }
            else if operation_matches(0b0010_0001, self.opcode) { Some(Condition::NotEqual) }
            else if operation_matches(0b0010_0010, self.opcode) { Some(Condition::Less) }
            else if operation_matches(0b0010_0011, self.opcode) { Some(Condition::LessEqual) }
            else if operation_matches(0b0010_0100, self.opcode) { Some(Condition::Greater) }
            else if operation_matches(0b0010_0101, self.opcode) { Some(Condition::GreaterEqual) }
            else { None }
        ;

        InstFlags {
            alu: AluOperation::decode(self.opcode),
            left_reg,
            right_reg,
            dest,
            prom_loading,
            ram_loading,
            is_save,
            call,
            ret,
            cond
        }
    }
}

use crate::processor::alu::AluOperation;
use crate::processor::branch::Condition;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct InstFlags {
    pub alu: Option<AluOperation>,
    pub ram_loading: bool,
    pub prom_loading: bool,
    pub is_save: bool,
    pub call: bool,
    pub ret: bool,
    pub cond: Option<Condition>,
    pub left_reg: Option<u8>,
    pub right_reg: Option<u8>,
    pub dest: Option<u8>
}

impl InstFlags {
    pub fn new() -> InstFlags {
        InstFlags {
            alu: None,
            ram_loading: false,
            prom_loading: false,
            is_save: false,
            call: false,
            ret: false,
            cond: None,
            left_reg: Some(0),
            right_reg: Some(0),
            dest: Some(0)
        }
    }
}

fn matches(pattern: u8, byte: u8) -> bool {
    byte & pattern == pattern
}

fn operation_matches(pattern: u8, byte: u8) -> bool {
    (byte & 0b0011_1111) ^ pattern == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_mov(){
        let inst = Instruction { 
            opcode: 64, 
            arg1: 7, 
            arg2: 0,
            arg3: 1
        };
        let flags = inst.decode();
        let true_flags = InstFlags {
            alu: Some(AluOperation::Or),
            left_reg: Some(7),
            right_reg: None,
            dest: Some(1),
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);
    }

    #[test]
    fn decode_load(){
        let load_inst = Instruction { 
            opcode: 24, 
            arg1: 0, 
            arg2: 0,
            arg3: 0
        };
        let prom_inst = Instruction { 
            opcode: 25, 
            arg1: 0, 
            arg2: 0,
            arg3: 0
        };
        let flags = load_inst.decode();
        let true_flags = InstFlags {
            ram_loading: true,
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);

        let flags = prom_inst.decode();
        let true_flags = InstFlags {
            prom_loading: true,
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);
    }

    #[test]
    fn decode_save(){
        let inst = Instruction { 
            opcode: 16, 
            arg1: 0, 
            arg2: 0,
            arg3: 0
        };
        let flags = inst.decode();
        let true_flags = InstFlags {
            is_save: true,
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);
    }

    #[test]
    fn decode_fun(){
        let call_inst = Instruction { 
            opcode: 38, 
            arg1: 0, 
            arg2: 0,
            arg3: 0
        };
        let ret_inst = Instruction { 
            opcode: 39, 
            arg1: 0, 
            arg2: 0,
            arg3: 0
        };
        let flags = call_inst.decode();
        let true_flags = InstFlags {
            call: true,
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);

        let flags = ret_inst.decode();
        let true_flags = InstFlags {
            ret: true,
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);
    }

    #[test]
    fn decode_be() {
        let inst = Instruction { 
            opcode: 96, 
            arg1: 0, 
            arg2: 0,
            arg3: 0
        };
        let flags = inst.decode();
        let true_flags = InstFlags {
            cond: Some(Condition::Equal),
            right_reg: None,
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);
    }

    #[test]
    fn decode_sub() {
        let inst = Instruction { 
            opcode: 67, 
            arg1: 0, 
            arg2: 1,
            arg3: 0
        };
        let flags = inst.decode();
        let true_flags = InstFlags {
            alu: Some(AluOperation::Sub),
            right_reg: None,
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);
    }
}