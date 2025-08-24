#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
}

impl TryFrom<u8> for OpCode {
    type Error = String;
    
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0 => Ok(OpCode::Return),
            1 => Ok(OpCode::Constant),
            _ => Err(format!("Unknown opcode: {}", byte)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}