use crate::scanner::{Scanner, Token, TokenType};
use crate::chunk::Chunk;
use crate::opcode::OpCode;
use crate::common::Value;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None = 0,
    Assignment = 1,  // =
    Or = 2,          // or (binary)
    And = 3,         // and (binary) 
    Equality = 4,    // == != (binary)
    Comparison = 5,  // < > <= >= (binary)
    Term = 6,        // + - (binary)
    Factor = 7,      // * / (binary)
    Unary = 8,       // ! - (unary operators)
    Call = 9,        // . () 
    Primary = 10,
}

impl Precedence {
    fn next(self) -> Self {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

type ParseFn = fn(&mut Compiler);

struct ParseRule {
    prefix: Option<ParseFn>,
    // the function to compile a prefix expression starting with a token of that type.
    infix: Option<ParseFn>,
    // the function to compile an infix expression whose left operand is followed by a token of that type.
    precedence: Precedence,
    // the precedence of an infix expression that uses that token as an operator.
}

struct Parser {
    current: Token, // Token we are going to process
    previous: Token, // Token we just consumed
    had_error: bool, // Did we encounter any errors?
    panic_mode: bool, // Are we in error recovery mode?
}

impl Parser {
    fn new() -> Self {
        Self {
            current: Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                line: 0,
            },
            previous: Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                line: 0,
            },
            had_error: false,
            panic_mode: false,
        }
    }
}

struct Compiler {
    scanner: Scanner,
    parser: Parser,
    compiling_chunk: Chunk,
}

impl Compiler {
    fn new(scanner: Scanner) -> Self {
        Self {
            scanner,
            parser: Parser::new(),
            compiling_chunk: Chunk::new(),
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }

    fn advance(&mut self) {
        // Move previous = current, current = next token from scanner
        // Handle error tokens here
        self.parser.previous = self.parser.current.clone();
        
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error {
                break;
            }
            
            self.error_at_current(&self.parser.current.lexeme.clone());
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.parser.current.clone(), message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.parser.previous.clone(), message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing
        } else {
            eprint!(" at '{}'", token.lexeme);
        }

        eprintln!(": {}", message);
        self.parser.had_error = true;
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        // Consume a token of the expected type, or report an error
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.parser.previous.line;
        self.current_chunk().write(byte, line);
    }

    fn emit_opcode(&mut self, opcode: OpCode) {
        let line = self.parser.previous.line;
        self.current_chunk().write_opcode(opcode, line);
    }

    fn emit_constant(&mut self, value: Value) {
        let line = self.parser.previous.line;
        self.current_chunk().write_constant(value, line);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        // This is the core of Pratt parsing!
        // 1. Advance and get the prefix rule for the current token
        self.advance();
        // 2. Call the prefix function
        let prefix_rule = Self::get_rule(self.parser.previous.token_type).prefix;
        
        match prefix_rule {
            Some(prefix_fn) => prefix_fn(self),
            None => {
                self.error("Expect expression.");
                return;
            }
        }
        // 3. While we have infix operators with higher precedence:
        //    - Get the infix rule and call the infix function
        while precedence <= Self::get_rule(self.parser.current.token_type).precedence {
            self.advance();
            let infix_rule = Self::get_rule(self.parser.previous.token_type).infix;
            if let Some(infix_fn) = infix_rule {
                infix_fn(self);
            }
        }
    }

    fn number(&mut self) {
        let value: f64 = self.parser.previous.lexeme.parse()
            .expect("Failed to parse number");
        self.emit_constant(value);
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        // Compile the operand
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction
        match operator_type {
            TokenType::Minus => self.emit_opcode(OpCode::Negate),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        // Parse binary expressions: left + right, left * right, etc.
        let operator_type = self.parser.previous.token_type;
        let rule = Self::get_rule(operator_type);
        
        // Compile the right operand with higher precedence
        // Why do we use next_precedence here?
        let next_precedence = rule.precedence.next();
        self.parse_precedence(next_precedence);
        
        // Emit the operator instruction
        match operator_type {
            TokenType::Plus => self.emit_opcode(OpCode::Add),
            TokenType::Minus => self.emit_opcode(OpCode::Subtract),
            TokenType::Star => self.emit_opcode(OpCode::Multiply),
            TokenType::Slash => self.emit_opcode(OpCode::Divide),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        // Parse parenthesized expressions: ( expression )
        // We assume the initial ( has already been consumed
        // because we can call this grouping after we know that we have '('
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn get_rule(token_type: TokenType) -> &'static ParseRule {
        use TokenType::*;
        match token_type {
            LeftParen => &ParseRule { 
                prefix: Some(Compiler::grouping), 
                infix: None, 
                precedence: Precedence::None 
            },
            Number => &ParseRule { 
                prefix: Some(Compiler::number), 
                infix: None, 
                precedence: Precedence::None 
            },
            Minus => &ParseRule { 
                prefix: Some(Compiler::unary),  // We can see sth like -1
                infix: Some(Compiler::binary),  // We can see sth like 1 - 5
                precedence: Precedence::Term    // Minus has a precedence of "Term"
            },
            Plus => &ParseRule { 
                prefix: None, 
                infix: Some(Compiler::binary), 
                precedence: Precedence::Term    // Plus has a precedence of "Term" 
            },
            Star => &ParseRule { 
                prefix: None, // We don't see *5
                infix: Some(Compiler::binary), // We can see 5 * 9
                precedence: Precedence::Factor  // Star has a precedence of "Factor" 
            },
            Slash => &ParseRule { 
                prefix: None, 
                infix: Some(Compiler::binary), 
                precedence: Precedence::Factor  // Slash has a precedence of "Factor" 
            },
            _ => &ParseRule { 
                prefix: None, 
                infix: None, 
                precedence: Precedence::None 
            },
        }
    }
}

pub fn compile(source: String) -> Option<Chunk> {
    let scanner = Scanner::new(source);
    let mut compiler = Compiler::new(scanner);
    
    compiler.advance();
    compiler.expression();
    compiler.consume(TokenType::Eof, "Expect end of expression.");
    compiler.emit_opcode(OpCode::Return);

    if compiler.parser.had_error {
        None
    } else {
        Some(compiler.compiling_chunk)
    }
}