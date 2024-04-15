use crate::PC;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub arg1: u8,
    pub arg2: u8,
    pub arg3: u8
}

impl Instruction {
    pub fn decode(&self) -> InstFlags {
        let left;
        let right;

        if matches(0b1000_0000, self.opcode) {
            left = (self.arg1, true);
        } else {
            left = (self.arg1, false);
            
        }

        if matches(0b0100_0000, self.opcode) {
            right = (self.arg2, true);
        } else {
            right = (self.arg2, false);
        }

        let prom_loading = operation_matches(0b0001_1001, self.opcode);
        let ram_loading = operation_matches(0b0001_1000, self.opcode);
        let save = operation_matches(0b0001_0000, self.opcode);
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

        let dest = if save || ret { None } else { Some(self.arg3) };

        let alu_op = AluOperation::decode(self.opcode);

        let jumped = call || cond.is_some() || (alu_op.is_some() || ram_loading || prom_loading) && dest == Some(PC);

        InstFlags {
            alu_op,
            left,
            right,
            dest,
            prom_loading,
            ram_loading,
            save,
            call,
            ret,
            cond,
            jumped
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct InstFlags {
    pub alu_op: Option<AluOperation>,
    pub ram_loading: bool,
    pub prom_loading: bool,
    pub save: bool,
    pub call: bool,
    pub ret: bool,
    pub cond: Option<Condition>,
    pub left: (u8, bool), // (val, is_immediate)
    pub right: (u8, bool), // (val, is_immediate)
    pub dest: Option<u8>,
    pub jumped: bool
}

impl InstFlags {
    #[allow(dead_code)] // Not actually dead because it is used in tests!
    pub fn new() -> InstFlags {
        InstFlags {
            alu_op: None,
            ram_loading: false,
            prom_loading: false,
            save: false,
            call: false,
            ret: false,
            cond: None,
            left: (0, false),
            right: (0, false),
            dest: None,
            jumped: false
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Condition {
    Equal,
    NotEqual, 
    Less,
    Greater,
    LessEqual,
    GreaterEqual
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum AluOperation {
    Or,
    And,
    Add,
    Sub,
    Not,
    Xor,
    LeftShift,
    RightShift,
    Mod,
    Div
}

impl AluOperation{
    pub fn decode(opcode: u8) -> Option<AluOperation> {
        let opcode = opcode & 0b0011_1111;
        match opcode {
            0b0 => Some(AluOperation::Or),
            0b1 => Some(AluOperation::And),
            0b10 => Some(AluOperation::Add),
            0b11 => Some(AluOperation::Sub),
            0b100 => Some(AluOperation::Not),
            0b101 => Some(AluOperation::Xor),
            0b1000 => Some(AluOperation::LeftShift),
            0b1001 => Some(AluOperation::RightShift),
            0b1100 => Some(AluOperation::Mod),
            0b1101 => Some(AluOperation::Div),
            _ => None
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
            alu_op: Some(AluOperation::Or),
            left: (7, false),
            right: (0, true),
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
            dest: Some(0),
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);

        let flags = prom_inst.decode();
        let true_flags = InstFlags {
            prom_loading: true,
            dest: Some(0),
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
            save: true,
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
            jumped: true,
            dest: Some(0),
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);

        let flags = ret_inst.decode();
        let true_flags = InstFlags {
            ret: true,
            jumped: false,
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
            right: (0, true),
            dest: Some(0),
            jumped: true,
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
            alu_op: Some(AluOperation::Sub),
            right: (1, true),
            dest: Some(0),
            ..InstFlags::new()
        };
        assert_eq!(flags, true_flags);
    }
}