use crate::Value;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Default)]
pub enum Token {
    // Basic operations
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,

    // Syntax-specific tokens
    Colon,
    Semi,

    // Primitive types
    Num(Value),
    Word(String),

    // Custom tokens
    #[default]
    Invalid,
}

impl std::convert::From<&str> for Token {
    fn from(s: &str) -> Self {
        match s {
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/" => Token::Div,
            "dup" => Token::Dup,
            "drop" => Token::Drop,
            "swap" => Token::Swap,
            "over" => Token::Over,
            ":" => Token::Colon,
            ";" => Token::Semi,
            _ => match s.parse::<Value>() {
                Ok(num) => Token::Num(num),
                Err(_) => Token::Word(s.to_string()),
            },
        }
    }
}

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, code: &str) -> Vec<Token> {
        let tokens: Vec<Token> = code
            .split_whitespace()
            .map(|chunk| Token::from(chunk.to_lowercase().as_str()))
            .collect();

        tokens
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
