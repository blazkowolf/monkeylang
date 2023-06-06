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
