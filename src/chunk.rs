use crate::common::Value;
use crate::opcode::OpCode;
use crate::value::ValueArray;

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>, // The bytecode instructions
    constants: ValueArray, // Pool of literal values
    lines: Vec<usize>, // Line number of a particular bytecode
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_opcode(&mut self, opcode: OpCode, line: usize) {
        self.write(opcode.into(), line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        let index = self.constants.count();
        self.constants.write(value);
        index
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        let constant_index = self.add_constant(value);
        if constant_index < 256 {
            self.write_opcode(OpCode::Constant, line);
            self.write(constant_index as u8, line);
        } else {
            panic!("Too many constants in one chunk ({})", constant_index);
        }
    }

    // Getters
    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn lines(&self) -> &[usize] {
        &self.lines
    }

    pub fn constants(&self) -> &ValueArray {
        &self.constants
    }

    pub fn count(&self) -> usize {
        self.code.len()
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}