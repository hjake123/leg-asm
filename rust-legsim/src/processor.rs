pub mod alu {
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum AluOperation {
        Or,
        And,
        Add,
        Sub,
        Not,
        Xor,
        MultHigh,
        MultLow,
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
                0b110 => Some(AluOperation::MultHigh),
                0b111 => Some(AluOperation::MultLow),
                0b1000 => Some(AluOperation::LeftShift),
                0b1001 => Some(AluOperation::RightShift),
                0b1100 => Some(AluOperation::Mod),
                0b1101 => Some(AluOperation::Div),
                _ => None
            }
        }
    }
}

pub mod branch {
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
}

pub mod mem {

}

#[cfg(test)]
mod tests {
    use super::*;
}