#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String),
    Int,
    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, ch: u8) -> Self {
        Self {
            token_type,
            literal: String::from_utf8(vec![ch]).expect("Token literal must be valid ASCII"),
        }
    }
}
