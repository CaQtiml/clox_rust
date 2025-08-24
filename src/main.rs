mod common;
mod chunk;
mod debug;
mod value;
mod test_helpers;
mod opcode;
// mod memory;

use chunk::{Chunk};
use debug::disassemble_chunk;
use crate::opcode::OpCode;
use test_helpers::{create_simple_chunk, verify_chunk_structure};

fn main() {
    let mut chunk = Chunk::new();
    
    // Test with a constant and return instruction
    // Because we don't have a scanner yet, we hard-code it.
    // Intend to be the statement `1.2;` at the line 123
    chunk.write_constant(1.2, 123); // 2 bytes including OP_CODE and OPERAND
    chunk.write_opcode(OpCode::Return, 123);
    
    disassemble_chunk(&chunk, "test chunk");

    let chunk = create_simple_chunk();
    verify_chunk_structure(&chunk);
    disassemble_chunk(&chunk, "test chunk");
}