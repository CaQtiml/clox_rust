#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    
    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    // Literals
    Identifier, String, Number,
    
    // Keywords
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,
    
    Error, Eof
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub struct Scanner {
    source: String,
    chars: Vec<char>, // The vector of source code's character
    start: usize, // Start of current token being scanned
    // Example: When scanning "print", start points to 'p'
    current: usize, // Current position in the source
    line: usize, // Current line number -> For error reporting
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars: Vec<char> = source.chars().collect();
        Scanner {
            source,
            chars,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        
        let c = self.advance();
        
        match c {
            // Single character tokens
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            
            // Two character tokens
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            },
            
            // String literals
            '"' => self.string(),
            
            // Numbers
            c if c.is_ascii_digit() => self.number(),
            
            // Identifiers and keywords
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            
            _ => self.error_token("Unexpected character."),
        }
    }
    
    // Helper methods
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }
    
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let c = self.chars[self.current];
            self.current += 1;
            c
        }
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }
    
    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: self.chars[self.start..self.current].iter().collect(),
            line: self.line,
        }
    }
    
    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            lexeme: message.to_string(),
            line: self.line,
        }
    }
    
    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' if self.peek_next() == '/' => {
                    // Comment goes to end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                },
                _ => break,
            }
        }
    }
    
    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        
        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }
        
        // Consume the closing quote
        self.advance();
        self.make_token(TokenType::String)
    }
    
    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        
        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the '.'
            self.advance();
            
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        
        self.make_token(TokenType::Number)
    }
    
    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        
        let token_type = self.identifier_type();
        self.make_token(token_type)
    }
    
    // Keyword recognition using the trie approach from the book
    fn identifier_type(&self) -> TokenType {
        if self.current <= self.start {
            return TokenType::Identifier;
        }
        
        match self.chars[self.start] {
            'a' => self.check_keyword(1, "nd", TokenType::And),
            'c' => self.check_keyword(1, "lass", TokenType::Class),
            'e' => self.check_keyword(1, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.chars[self.start + 1] {
                        'a' => self.check_keyword(2, "lse", TokenType::False),
                        'o' => self.check_keyword(2, "r", TokenType::For),
                        'u' => self.check_keyword(2, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            },
            'i' => self.check_keyword(1, "f", TokenType::If),
            'n' => self.check_keyword(1, "il", TokenType::Nil),
            'o' => self.check_keyword(1, "r", TokenType::Or),
            'p' => self.check_keyword(1, "rint", TokenType::Print),
            'r' => self.check_keyword(1, "eturn", TokenType::Return),
            's' => self.check_keyword(1, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.chars[self.start + 1] {
                        'h' => self.check_keyword(2, "is", TokenType::This),
                        'r' => self.check_keyword(2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            },
            'v' => self.check_keyword(1, "ar", TokenType::Var),
            'w' => self.check_keyword(1, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }
    
    fn check_keyword(&self, start_offset: usize, rest: &str, token_type: TokenType) -> TokenType {
        let length = rest.len();
        if self.current - self.start == start_offset + length {
            let slice: String = self.chars[self.start + start_offset..self.current].iter().collect();
            if slice == rest {
                return token_type;
            }
        }
        TokenType::Identifier
    }
}