use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::common::Value;

pub fn create_simple_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    chunk.write_constant(Value::Number(1.2), 123);
    chunk.write_opcode(OpCode::Negate, 123);
    chunk.write_opcode(OpCode::Return, 123);
    chunk
}

pub fn create_arithmetic_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    
    chunk.write_constant(Value::Number(1.2), 123);
    chunk.write_constant(Value::Number(3.4), 123);
    chunk.write_opcode(OpCode::Add, 123);
    chunk.write_opcode(OpCode::Negate, 123);
    chunk.write_opcode(OpCode::Return, 123);
    
    chunk
}

pub fn create_complex_arithmetic_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    
    chunk.write_constant(Value::Number(1.0), 123);
    chunk.write_constant(Value::Number(2.0), 123);
    chunk.write_opcode(OpCode::Add, 123);
    
    chunk.write_constant(Value::Number(3.0), 123);
    chunk.write_opcode(OpCode::Multiply, 123);
    
    chunk.write_constant(Value::Number(4.0), 123);
    chunk.write_opcode(OpCode::Subtract, 123);

    chunk.write_constant(Value::Number(4.0), 123);
    chunk.write_opcode(OpCode::Divide, 123);
    
    chunk.write_opcode(OpCode::Return, 123);
    
    chunk
}

pub fn create_complex_arithmetic_divide_zero_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    
    chunk.write_constant(Value::Number(1.0), 123);
    chunk.write_constant(Value::Number(2.0), 123);
    chunk.write_opcode(OpCode::Add, 123);
    
    chunk.write_constant(Value::Number(3.0), 123);
    chunk.write_opcode(OpCode::Multiply, 123);
    
    chunk.write_constant(Value::Number(4.0), 123);
    chunk.write_opcode(OpCode::Subtract, 123);

    chunk.write_constant(Value::Number(0.0), 123);
    chunk.write_opcode(OpCode::Divide, 123);
    
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