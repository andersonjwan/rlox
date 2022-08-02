pub mod tokens;

use core::fmt;
use std::error;

use tokens::{Position, Token, TokenKind};

#[derive(Debug)]
struct LexerError;

impl error::Error for LexerError {}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lexical analysis error")
    }
}

struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while (!self.end()) {
            self.start = self.current;

            match self.tokenize() {
                Ok(token) => tokens.push(token),
                Err(e) => eprintln!("{}", e),
            }
        }

        tokens.push(Token::new(TokenKind::EndOfFile, None, Position(0, 0)));
        tokens
    }

    fn tokenize(&mut self) -> Result<Token, LexerError> {
        match self.next() {
            '(' => Ok(self.construct(TokenKind::LeftParen)),
            ')' => Ok(self.construct(TokenKind::RightParen)),
            '{' => Ok(self.construct(TokenKind::LeftBrace)),
            '}' => Ok(self.construct(TokenKind::RightBrace)),
            ',' => Ok(self.construct(TokenKind::Comma)),
            '.' => Ok(self.construct(TokenKind::Dot)),
            '-' => Ok(self.construct(TokenKind::Minus)),
            '+' => Ok(self.construct(TokenKind::Plus)),
            ';' => Ok(self.construct(TokenKind::Semicolon)),
            '*' => Ok(self.construct(TokenKind::Star)),
            _ => Err(LexerError),
        }
    }

    fn construct(&self, kind: TokenKind) -> Token {
        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        Token::new(kind, Some(lexeme), Position(self.start, self.line))
    }

    fn next(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn end(&self) -> bool {
        self.current >= self.source.len()
    }
}
