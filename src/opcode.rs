#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
    Negate = 2,
    Add = 3,
    Subtract = 4,
    Multiply = 5,
    Divide = 6,
    Nil = 7,
    True = 8,
    False = 9,
    Not = 10,           // Logical NOT (!)
    Equal = 11,         // ==
    Greater = 12,       // >
    Less = 13,          // 
}

impl TryFrom<u8> for OpCode {
    type Error = String;
    
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0 => Ok(OpCode::Return),
            1 => Ok(OpCode::Constant),
            2 => Ok(OpCode::Negate),
            3 => Ok(OpCode::Add),
            4 => Ok(OpCode::Subtract),
            5 => Ok(OpCode::Multiply),
            6 => Ok(OpCode::Divide),
            7 => Ok(OpCode::Nil),
            8 => Ok(OpCode::True),
            9 => Ok(OpCode::False),
            10 => Ok(OpCode::Not),
            11 => Ok(OpCode::Equal),
            12 => Ok(OpCode::Greater),
            13 => Ok(OpCode::Less),
            _ => Err(format!("Unknown opcode: {}", byte)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}