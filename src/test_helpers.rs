use crate::chunk::Chunk;
use crate::opcode::OpCode;

pub fn create_simple_chunk() -> Chunk {
    let mut chunk = Chunk::new();
    let c = chunk.add_constant(1.2);
    chunk.write_opcode(OpCode::Constant, 123);
    chunk.write(c as u8, 123);
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