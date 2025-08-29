use crate::common::Value;

#[derive(Debug)]
pub struct ValueArray {
    values: Vec<Value>,
}

// An array that holds Value objects.
/*
print 3.14;
print 2.71;
print 42;

Index | Value
------|------
  0   | 3.14
  1   | 2.71  
  2   | 42

OP_CONSTANT 0    // Load constant at index 0 (3.14)
OP_PRINT         // Print it
OP_CONSTANT 1    // Load constant at index 1 (2.71)
OP_PRINT         // Print it
OP_CONSTANT 2    // Load constant at index 2 (42)
OP_PRINT         // Print it
*/
// The reason we don't store the actual value directly to the instruction
// is that values can be too large
// and variable-length instruction is difficult to handle.
impl ValueArray {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    pub fn values(&self) -> &[Value] {
        &self.values
    }
}

impl Default for ValueArray {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_value(value: &Value) {
    println!("{}", value);
}

// Helper function for formatting values in disassembly
pub fn value_to_string(value: Value) -> String {
    format!("{}", value)
}