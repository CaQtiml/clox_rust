use crate::{chunk::Chunk, opcode::OpCode};
use crate::common::Value;
use crate::value::print_value;
use crate::debug::{disassemble_instruction};
use anyhow::{bail, Result};

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
        self.reset_stack();
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
                        self.push(constant)?;
                    }
                    OpCode:: True => {
                        print_value(&Value::Bool(true));
                        self.push(Value::Bool(true))?;
                    }
                    OpCode:: False => {
                        print_value(&Value::Bool(false));
                        self.push(Value::Bool(false))?;
                    }
                    OpCode:: Nil => {
                        print_value(&Value::Nil);
                        self.push(Value::Nil)?;
                    }
                    OpCode::Return => {
                        let value = self.pop()?;
                        print_value(&value);
                        println!();
                        return Ok(InterpretResult::Ok);
                    }
                    OpCode::Negate => {
                        let value = self.pop()?;
                        // let negate_value = -value;
                        // print_value(negate_value);
                        // self.push(negate_value)?;
                        match value {
                            Value::Number(x) => {
                                print_value(&Value::Number(-x));
                                self.push(Value::Number(-x));
                            }
                            _ => bail!("Operand must be a number."),
                        }
                    }
                    OpCode::Add => {
                        self.binary_op_with_check(|a, b| match (a, b) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                            _ => Err("Operands must be numbers.".to_string()),
                        })?;
                    }
                    OpCode::Subtract => {
                        self.binary_op_with_check(|a, b| match (a, b) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                            _ => Err("Operands must be numbers.".to_string()),
                        })?;
                    }
                    OpCode::Multiply => {
                        self.binary_op_with_check(|a, b| match (a, b) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                            _ => Err("Operands must be numbers.".to_string()),
                        })?;
                    }
                    OpCode::Divide => {
                        self.binary_op_with_check(|a, b| match (a, b) {
                            (Value::Number(a), Value::Number(b)) => {
                                if b == 0.0 {
                                    Err("Division by zero".to_string())
                                } else {
                                    Ok(Value::Number(a / b))
                                }
                            },
                            _ => Err("Operands must be numbers.".to_string()),
                        })?;
                    }
                    OpCode:: Not => {
                        let value = self.pop()?;
                        self.push(Value::Bool(value.is_falsy()))?;
                    }
                    OpCode:: Greater => {
                        self.binary_op_with_check(|a, b| match (a, b) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                            _ => Err("Operands must be numbers.".to_string()),
                        })?;
                    }
                    OpCode:: Less => {
                        self.binary_op_with_check(|a, b| match (a, b) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                            _ => Err("Operands must be numbers.".to_string()),
                        })?;
                    }
                    OpCode:: Equal => {
                        let b = self.pop()?;
                        let a = self.pop()?;
                        self.push(Value::Bool(a == b))?;
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
        chunk.constants().get(constant_index).expect("Invalid constant index").clone()
    }

    fn push(&mut self, value: Value) -> Result<()> {
        if self.stack.len() >= STACK_MAX {
            bail!("Stack overflow");
        }
        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or_else(|| anyhow::anyhow!("Stack underflow"))
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn binary_op_with_check<F>(&mut self, op: F) -> Result<()>
    where
        F: FnOnce(Value, Value) -> Result<Value, String>,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        match op(a, b) {
            Ok(result) => self.push(result)?,
            Err(error) => bail!("{}", error),
        }
        Ok(())
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