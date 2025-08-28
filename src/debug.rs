use crate::chunk::{Chunk};
use crate::value::print_value;
use crate::opcode::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    
    let mut offset = 0;
    while offset < chunk.count() {
        offset = disassemble_instruction(chunk, offset);
    }
}

// Given a chunk, it will print out all of the instructions in it.
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset); 
    // {:04} makes the output exactly 4 characters wide
    // If the number is shorter than 4 characters, pad it with zeros on the left
    
    // Show line information
    /*
        print 1 + 2 * 3;

        0000  123 OP_CONSTANT        0 '1'      // Load constant 1
        0002    | OP_CONSTANT        1 '2'      // Load constant 2  
        ...
    */
    if offset > 0 && chunk.lines()[offset] == chunk.lines()[offset - 1] {
        print!("   | "); // Print "|" if this bytecode is from the same line as the previous bytecode
    } else {
        print!("{:4} ", chunk.lines()[offset]); // If not, print the line number
    }
    
    let instruction = chunk.code()[offset];
    
    // Handle potential invalid opcodes gracefully
    match OpCode::try_from(instruction) {
        Ok(opcode) => match opcode {
            OpCode::Return => simple_instruction("OP_RETURN", offset),
            OpCode::Constant => constant_instruction("OP_CONSTANT", chunk, offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
        },
        Err(_) => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1 // RETURN as a size of 1 byte
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code()[offset + 1] as usize;
    print!("{:<16} {:4} '", name, constant_index);
    
    if let Some(value) = chunk.constants().get(constant_index) {
        print_value(*value);
    } else {
        print!("INVALID_CONSTANT");
    }
    
    // println!("'");
    offset + 2 // Because CONSTANT has a size of 2 bytes.
}