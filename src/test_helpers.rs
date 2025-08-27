use crate::chunk::Chunk;
use crate::opcode::OpCode;

pub fn create_simple_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    // Test with a constant and return instruction
    // Because we don't have a scanner yet, we hard-code it.
    // Intend to be the statement `1.2;` at the line 123
    chunk.write_constant(1.2, 123); // 2 bytes including OP_CODE and OPERAND
    chunk.write_opcode(OpCode::Negate, 123);
    chunk.write_opcode(OpCode::Return, 123);
    chunk
}

pub fn create_arithmetic_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    
    // Load 1.2
    chunk.write_constant(1.2, 123);
    
    // Load 3.4
    chunk.write_constant(3.4, 123);
    
    // Add them
    chunk.write_opcode(OpCode::Add, 123);
    
    // Negate the result
    chunk.write_opcode(OpCode::Negate, 123);
    
    // Return
    chunk.write_opcode(OpCode::Return, 123);
    
    chunk
}

pub fn create_complex_arithmetic_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    
    chunk.write_constant(1.0, 123);
    chunk.write_constant(2.0, 123);
    chunk.write_opcode(OpCode::Add, 123);      // (1 + 2) -> 3
    
    chunk.write_constant(3.0, 123);
    chunk.write_opcode(OpCode::Multiply, 123); // 3 * 3 -> 9
    
    chunk.write_constant(4.0, 123);
    chunk.write_opcode(OpCode::Subtract, 123); // 9 - 4 -> 5

    chunk.write_constant(4.0, 123);
    chunk.write_opcode(OpCode::Divide, 123); // 5/4=1.25
    
    chunk.write_opcode(OpCode::Return, 123);
    
    chunk
}

pub fn create_complex_arithmetic_divide_zero_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    
    chunk.write_constant(1.0, 123);
    chunk.write_constant(2.0, 123);
    chunk.write_opcode(OpCode::Add, 123);      // (1 + 2) -> 3
    
    chunk.write_constant(3.0, 123);
    chunk.write_opcode(OpCode::Multiply, 123); // 3 * 3 -> 9
    
    chunk.write_constant(4.0, 123);
    chunk.write_opcode(OpCode::Subtract, 123); // 9 - 4 -> 5

    chunk.write_constant(0.0, 123);
    chunk.write_opcode(OpCode::Divide, 123); // 5/0 ERROR
    
    chunk.write_opcode(OpCode::Return, 123);
    
    chunk
}

pub fn verify_chunk_structure(chunk: &Chunk) {
    println!("Chunk Statistics:");
    println!("  Code size: {} bytes", chunk.count());
    println!("  Constants: {}", chunk.constants().count());
    println!("  Raw bytes: {:?}", chunk.code());
    println!();
}