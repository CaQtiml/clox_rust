mod common;
mod chunk;
mod debug;
mod value;
mod test_helpers;
mod opcode;
// mod memory;
mod vm;
mod compiler;
mod scanner;

use chunk::{Chunk};
use debug::disassemble_chunk;
use crate::opcode::OpCode;
use test_helpers::{create_simple_chunk, verify_chunk_structure};
use test_helpers::{create_arithmetic_chunk, create_complex_arithmetic_chunk, create_complex_arithmetic_divide_zero_chunk};
use vm::{VM, InterpretResult};

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut vm = VM::new();
    
    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => {
            eprintln!("Usage: {} [path]", args[0]);
            process::exit(64);
        }
    }
}

fn repl(vm: &mut VM) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            },
            Ok(_) => {
                run_source(vm, line.trim().to_string());
            },
            Err(_) => {
                println!();
                break;
            }
        }
    }
}

fn run_file(vm: &mut VM, path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Could not open file \"{}\".", path);
            process::exit(74);
        }
    };
    
    let result = run_source(vm, source);
    
    match result {
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
        InterpretResult::Ok => {},
    }
}

fn run_source(_vm: &mut VM, source: String) -> InterpretResult {
    compiler::compile(source);
    InterpretResult::Ok
}