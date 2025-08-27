mod common;
mod chunk;
mod debug;
mod value;
mod test_helpers;
mod opcode;
// mod memory;
mod vm;

use chunk::{Chunk};
use debug::disassemble_chunk;
use crate::opcode::OpCode;
use test_helpers::{create_simple_chunk, verify_chunk_structure};
use test_helpers::{create_arithmetic_chunk, create_complex_arithmetic_chunk, create_complex_arithmetic_divide_zero_chunk};
use vm::{VM, InterpretResult};

fn main() -> anyhow::Result<()> {
    println!("=== Basic Constant Test ===");
    test_simple_constant()?;
    
    println!("\n=== Arithmetic Test: -(1.2 + 3.4) ===");
    test_arithmetic()?;

    // println!("\n=== Complex Arithmetic Test Divided by Zero: ((1 + 2) * 3 - 4)/0 ===");
    // test_complex_arithmetic_divide_zero()?;
    
    println!("\n=== Complex Arithmetic Test: ((1 + 2) * 3 - 4)/4 ===");
    test_complex_arithmetic()?;

    
    
    Ok(())
}

fn test_simple_constant() -> anyhow::Result<()> {
    let chunk = create_simple_chunk();
    verify_chunk_structure(&chunk);
    
    println!("--- Disassembly ---");
    disassemble_chunk(&chunk, "simple constant");
    
    println!("--- VM Execution ---");
    let mut vm = VM::new();
    execute_chunk(&mut vm, chunk)?;
    
    Ok(())
}

fn test_arithmetic() -> anyhow::Result<()> {
    let chunk = create_arithmetic_chunk();
    verify_chunk_structure(&chunk);
    
    println!("--- Disassembly ---");
    disassemble_chunk(&chunk, "arithmetic test");
    
    println!("--- VM Execution ---");
    let mut vm = VM::new();
    execute_chunk(&mut vm, chunk)?;
    
    Ok(())
}

fn test_complex_arithmetic() -> anyhow::Result<()> {
    let chunk = create_complex_arithmetic_chunk();
    verify_chunk_structure(&chunk);
    
    println!("--- Disassembly ---");
    disassemble_chunk(&chunk, "complex arithmetic");
    
    println!("--- VM Execution ---");
    let mut vm = VM::new();
    execute_chunk(&mut vm, chunk)?;
    
    Ok(())
}

fn test_complex_arithmetic_divide_zero() -> anyhow::Result<()> {
    let chunk = create_complex_arithmetic_divide_zero_chunk();
    verify_chunk_structure(&chunk);
    
    println!("--- Disassembly ---");
    disassemble_chunk(&chunk, "complex arithmetic divided by zero");
    
    println!("--- VM Execution ---");
    let mut vm = VM::new();
    execute_chunk(&mut vm, chunk)?;
    
    Ok(())
}

fn execute_chunk(vm: &mut VM, chunk: Chunk) -> anyhow::Result<()> {
    match vm.interpret(chunk)? {
        InterpretResult::Ok => println!("✓ Execution completed successfully"),
        InterpretResult::RuntimeError => println!("✗ Runtime error occurred"),
        InterpretResult::CompileError => println!("✗ Compile error occurred"),
    }
    Ok(())
}