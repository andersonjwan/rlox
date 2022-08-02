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

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.end() {
            self.start = self.current;

            match self.tokenize() {
                Some(option) => match option {
                    Ok(token) => tokens.push(token),
                    Err(e) => eprintln!("{}", e),
                },
                None => continue,
            }
        }

        tokens.push(Token::new(
            TokenKind::EndOfFile,
            None,
            Position(self.current, self.line),
        ));
        tokens
    }

    fn tokenize(&mut self) -> Option<Result<Token, LexerError>> {
        match self.next() {
            '(' => Some(Ok(self.construct(TokenKind::LeftParen))),
            ')' => Some(Ok(self.construct(TokenKind::RightParen))),
            '{' => Some(Ok(self.construct(TokenKind::LeftBrace))),
            '}' => Some(Ok(self.construct(TokenKind::RightBrace))),
            ',' => Some(Ok(self.construct(TokenKind::Comma))),
            '.' => Some(Ok(self.construct(TokenKind::Dot))),
            '-' => Some(Ok(self.construct(TokenKind::Minus))),
            '+' => Some(Ok(self.construct(TokenKind::Plus))),
            ';' => Some(Ok(self.construct(TokenKind::Semicolon))),
            '*' => Some(Ok(self.construct(TokenKind::Star))),
            '!' => match self.peek(1) {
                Some('=') => Some(Ok(self.consume(TokenKind::BangEqual))),
                _ => Some(Ok(self.construct(TokenKind::Bang))),
            },
            '=' => match self.peek(1) {
                Some('=') => Some(Ok(self.consume(TokenKind::EqualEqual))),
                _ => Some(Ok(self.construct(TokenKind::Equal))),
            },
            '<' => match self.peek(1) {
                Some('=') => Some(Ok(self.consume(TokenKind::LessEqual))),
                _ => Some(Ok(self.construct(TokenKind::Less))),
            },
            '>' => match self.peek(1) {
                Some('=') => Some(Ok(self.consume(TokenKind::GreaterEqual))),
                _ => Some(Ok(self.construct(TokenKind::Greater))),
            },
            '/' => match self.peek(1) {
                Some('/') => {
                    while let Some(character) = self.peek(1) {
                        if character != '\n' {
                            self.next();
                            continue;
                        }

                        break;
                    }

                    None // nothing tokenized
                }
                _ => Some(Ok(self.construct(TokenKind::ForwardSlash))),
            },
            ' ' | '\r' | '\t' => None, // nothing tokenized
            '\n' => {
                self.line += 1;
                None // nothing tokenized
            }
            '"' => Some(self.stringify()),
            '0'..='9' => Some(self.numberify()),
            _ => Some(Err(LexerError)),
        }
    }

    fn stringify(&mut self) -> Result<Token, LexerError> {
        while let Some(character) = self.peek(1) {
            if character != '"' {
                if character == '\n' {
                    self.line += 1;
                }

                self.next();
                continue;
            }

            break;
        }

        self.next();

        if self.end() {
            return Err(LexerError);
        }

        let lexeme: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 2 - self.start)
            .collect();

        Ok(Token::new(
            TokenKind::String,
            Some(lexeme),
            Position(self.start + 1, self.line),
        ))
    }

    fn numberify(&mut self) -> Result<Token, LexerError> {
        while let Some(character) = self.peek(1) {
            if character.is_ascii_digit() {
                self.next();
                continue;
            }

            break;
        }

        if let Some('.') = self.peek(1) {
            self.next(); // consume dot

            while let Some(character) = self.peek(1) {
                if character.is_ascii_digit() {
                    self.next();
                    continue;
                }

                break;
            }
        }

        Ok(self.construct(TokenKind::Number))
    }

    fn consume(&mut self, kind: TokenKind) -> Token {
        self.current += 1;
        self.construct(kind)
    }

    fn peek(&self, distance: usize) -> Option<char> {
        if self.current + distance >= self.source.len() {
            None
        } else {
            Some(
                self.source
                    .chars()
                    .nth(self.current - 1 + distance)
                    .unwrap(),
            )
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
