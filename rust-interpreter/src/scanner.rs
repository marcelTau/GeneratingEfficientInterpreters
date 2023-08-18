use std::fmt;
use std::collections::HashMap;

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_alpha(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,

    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,
    Dot,
    Percent,

    Assignment, // :=

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,

    True,
    False,

    Identifier,
    NumberLiteral,

    /// Keywords
    While,
    Do,
    If,
    Then,
    End,
    Else,
    Print,
    Continue,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Bool(bool),
    Variable(String),
    DivByZeroError,
    ArithmeticError,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Bool(x) => write!(f, "{x}"),
            Object::Variable(x) => write!(f, "{x}"),
            Object::ArithmeticError => write!(f, "ArithmeticError"),
            Object::DivByZeroError => write!(f, "DivByZeroError"),
        }
    }
}

impl std::ops::Mul for Object {
    type Output = Object;

    fn mul(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left * right),
            _ => Object::ArithmeticError,
        }
    }
}

impl std::ops::Div for Object {
    type Output = Object;

    fn div(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => {
                if right == 0 as f64 {
                    Object::DivByZeroError
                } else {
                    Object::Num(left / right)
                }
            }
            _ => Object::ArithmeticError,
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Object;

    fn sub(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left - right),
            _ => Object::ArithmeticError,
        }
    }
}

impl std::ops::Add for Object {
    type Output = Object;

    fn add(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left + right),
            _ => Object::ArithmeticError,
        }
    }
}

impl std::ops::Rem for Object {
    type Output = Object;

    fn rem(self, other: Self) -> Object {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => Object::Num(left % right),
            _ => Object::ArithmeticError,
        }
    }
}

impl std::cmp::PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Num(left), Object::Num(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<Object>, // could be f64 aswell when not including error types
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<Object>) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

pub struct Scanner {
    tokens: Vec<Token>,
    source_code: String,
    start: usize,
    current: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source_code: &str) -> Self {
        let keywords = HashMap::from([
            ("while".to_string(), TokenType::While),
            ("do".to_string(), TokenType::Do),
            ("false".to_string(), TokenType::False),
            ("if".to_string(), TokenType::If),
            ("else".to_string(), TokenType::Else),
            ("then".to_string(), TokenType::Then),
            ("end".to_string(), TokenType::End),
            ("print".to_string(), TokenType::Print),
            ("true".to_string(), TokenType::True),
            ("false".to_string(), TokenType::False),
            ("continue".to_string(), TokenType::Continue),
        ]);
        Scanner {
            source_code: source_code.to_string(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            keywords
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source_code.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source_code.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let mut lexeme = &self.source_code[self.start..self.current];
        if token_type == TokenType::EOF {
            lexeme = "";
        }
        self.tokens.push(Token::new(
            token_type,
            literal,
        ));
    }

    fn add_token_single(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source_code.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source_code.chars().nth(self.current).unwrap();
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source_code.len() {
            return '\0';
        }
        self.source_code.chars().nth(self.current + 1).unwrap()
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source_code[self.start..self.current];
        let token_type = *self
            .keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier);

        match token_type {
            TokenType::True => self.add_token(TokenType::True, Some(Object::Bool(true))),
            TokenType::False => self.add_token(TokenType::False, Some(Object::Bool(false))),
            _ => self.add_token(token_type, Some(Object::Variable(text.to_string()))),
            //_ => self.add_token_single(token_type),
        }
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let literal: f64 = self.source_code[self.start..self.current]
            .to_string()
            .parse()
            .unwrap();
        self.add_token(TokenType::NumberLiteral, Some(Object::Num(literal)));
    }

    fn scan_token(&mut self) -> Result<(), ()> {
        let c: char = self.advance();

        match c {
            '(' => self.add_token_single(TokenType::LeftParen),
            ')' => self.add_token_single(TokenType::RightParen),
            '.' => self.add_token_single(TokenType::Dot),
            '-' => self.add_token_single(TokenType::Minus),
            '+' => self.add_token_single(TokenType::Plus),
            ';' => self.add_token_single(TokenType::Semicolon),
            '*' => self.add_token_single(TokenType::Star),
            '%' => self.add_token_single(TokenType::Percent),
            ':' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::Assignment)
                } else {
                    panic!("Assignment operator in scanner")
                }
            }
            '!' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::BangEqual)
                } else {
                    self.add_token_single(TokenType::Bang)
                }
            }
            '=' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::EqualEqual)
                } else {
                    self.add_token_single(TokenType::Equal)
                }
            }
            '<' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::LessEqual)
                } else {
                    self.add_token_single(TokenType::Less)
                }
            }
            '>' => {
                if self.expect('=') {
                    self.add_token_single(TokenType::GreaterEqual)
                } else {
                    self.add_token_single(TokenType::Greater)
                }
            }
            '&' => {
                if self.expect('&') {
                    self.add_token_single(TokenType::And)
                } else {
                    unreachable!("and scanner")
                }
            }
            '|' => {
                if self.expect('|') {
                    self.add_token_single(TokenType::Or)
                } else {
                    unreachable!("and scanner")
                }
            }
            '/' => {
                if self.expect('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.expect('*') {
                    while self.peek() != '*' && self.peek_next() != '/' {
                        self.advance();
                    }
                    // skip the */
                    self.advance();
                    self.advance();
                } else {
                    self.add_token_single(TokenType::Slash)
                }
            }

            ' ' | '\t' | '\r' | '\n' => {}

            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    unimplemented!();
                }
            }
        }
        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ()> {
        let emit_token = |token_type: TokenType| {
            let lexeme = &self.source_code[self.start..self.current];
        };

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token_single(TokenType::EOF);
        Ok(self.tokens.clone())
    }
}
