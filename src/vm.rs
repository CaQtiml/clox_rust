use crate::{chunk::Chunk, opcode::OpCode};
use crate::common::Value;
use crate::value::print_value;
use crate::debug::{disassemble_chunk, disassemble_instruction};
use anyhow::{Result, bail};

const STACK_MAX: usize = 256;

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,  // instruction pointer (or it is called Program Counter) - index into chunk.code()
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {

    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<InterpretResult> {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }
    
    fn run(&mut self) -> Result<InterpretResult> {
        loop {

            self.debug_trace_execution();

            // TODO: Read the next instruction
            let instruction: u8 = self.read_byte();
            // TODO: Match on the opcode and execute it
            match OpCode::try_from(instruction) {
                Ok(opcode) => match opcode {
                    OpCode::Constant => {
                        let constant = self.read_constant();
                        println!("Loading constant: {}", constant);
                    }
                    OpCode::Return => {
                        println!("Returning from VM");
                        return Ok(InterpretResult::Ok);
                    }
                },
                Err(_) => {
                    bail!("Unknown opcode: {}", instruction);
                }
            }
            // TODO: Handle different instruction types
        }
    }

    fn read_byte(&mut self) -> u8 {
        // TODO: Read byte at IP and advance IP
        let chunk = self.chunk.as_ref().expect("No chunk loaded");
        // .as_ref() is to change from Option(Chunk) to Option(&Chunk) 
        // so that it becomes &Chunk after unwrapping to make it borrowing instead of ownership taking
        let byte = chunk.code()[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        // TODO: Read a constant index and return the constant
        let constant_index = self.read_byte() as usize;
        // Don't forget that constant byte in `Chunk` is only an index refering to `constants`
        let chunk = self.chunk.as_ref().expect("No Chunk Loaded");
        *(chunk.constants().get(constant_index).expect("Invalid constant index"))
    }

    fn debug_trace_execution(&self) {
        // Print current stack state
        print!("          ");
        for value in &self.stack {
            print!("[ {} ]", value);
        }
        println!();
        
        // Disassemble current instruction
        if let Some(chunk) = &self.chunk {
            disassemble_instruction(chunk, self.ip);
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}