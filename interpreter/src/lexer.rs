use std::{iter::Peekable, str::Bytes};

use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    /// Source code string
    input: Peekable<Bytes<'a>>,
    /// Current position in `input` (points to current char)
    // position: usize,
    /// Current reading position in `input` (after current char)
    // read_position: usize,
    /// Current char under examination
    ch: u8,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Self {
            input: "".bytes().peekable(),
            // position: Default::default(),
            // read_position: Default::default(),
            ch: Default::default(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.bytes().peekable(),
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }
}

impl Lexer<'_> {
    /// Yield the next character and advance
    /// the position in the input string.
    fn read_char(&mut self) {
        self.ch = match self.input.peek() {
            Some(ch) => *ch,
            None => b'\0',
        };

        self.input.next();
    }

    pub fn next_token(&mut self) -> Token {
        use TokenType::*;

        let token = match self.ch {
            b'=' => Token::new(Assign, self.ch),
            b';' => Token::new(Semicolon, self.ch),
            b'(' => Token::new(LParen, self.ch),
            b')' => Token::new(RParen, self.ch),
            b',' => Token::new(Comma, self.ch),
            b'+' => Token::new(Plus, self.ch),
            b'{' => Token::new(LBrace, self.ch),
            b'}' => Token::new(RBrace, self.ch),
            b'\0' => Token {
                token_type: Eof,
                literal: "".to_string(),
            },
            _ => todo!("Not all Token types are implemented!"),
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        use TokenType::*;

        let input = "=+(){},;";
        let tests = vec![
            Token {
                token_type: Assign,
                literal: String::from("="),
            },
            Token {
                token_type: Plus,
                literal: String::from("+"),
            },
            Token {
                token_type: LParen,
                literal: String::from("("),
            },
            Token {
                token_type: RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: Comma,
                literal: String::from(","),
            },
            Token {
                token_type: Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: Eof,
                literal: String::from(""),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (
            i,
            Token {
                token_type,
                literal,
            },
        ) in tests.into_iter().enumerate()
        {
            let token = lexer.next_token();
            assert_eq!(
                token.token_type, token_type,
                "tests[{i}] - tokentype wrong. expected={:?}, got={token_type:?}",
                token.token_type
            );
            assert_eq!(
                token.literal, literal,
                "tests[{i}] - literal wrong. expected={:?}, got={literal:?}",
                token.literal
            );
        }
    }
}
