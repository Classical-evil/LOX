use crate::Token::Token;
use crate::TokenType::LiteralsValue;
use crate::TokenType::TokenType;
use crate::TokenType::KEYWORDS;
use crate::error;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        Scanner {
            source: src.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scanTokens(&mut self) -> Vec<Token> {
        while !self.isAtEnd() {
            self.start = self.current;
            self.scanToken();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            LiteralsValue::Nil,
            self.line,
        ));

        use std::mem;
        mem::take(&mut self.tokens)
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn addToken(&mut self, tokenType: TokenType) {
        self.addTokenWithLiteral(tokenType, LiteralsValue::Nil);
    }

    fn addTokenWithLiteral(&mut self, tokenType: TokenType, literal: LiteralsValue) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(tokenType, lexeme, literal, self.line));
    }

    fn matchChar(&mut self, expected: char) -> bool {
        if self.isAtEnd() {
            return false;
        }

        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        if c != expected {
            return false;
        }
        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.isAtEnd() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .expect("Line 74!!!!!!!!!!!")
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.isAtEnd() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.isAtEnd() {
            crate::error(self.line, "Unterminated string.");
            return;
        }
        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.addTokenWithLiteral(TokenType::String, LiteralsValue::String(value));
    }

    fn isDigit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn peekNext(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current + 1)
            .expect("PeekNext !!!!!!!!")
    }

    fn number(&mut self) {
        while self.isDigit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.isDigit(self.peekNext()) {
            self.advance();
            while self.isDigit(self.peek()) {
                self.advance();
            }
        }

        let numstr: &str = &self.source[self.start..self.current];
        let numValue = match numstr.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                crate::error(self.line, "Invail number.");
                0.0
            }
        };
        self.addTokenWithLiteral(TokenType::Number, LiteralsValue::Number(numValue));
    }

    fn isAlpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn isAlphaNumeric(&self, c: char) -> bool {
        self.isAlpha(c) || self.isDigit(c)
    }

    fn identifier(&mut self) {
        while self.isAlphaNumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text).copied().unwrap_or(TokenType::Identifier);
        self.addToken(token_type);
    }

    fn scanToken(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.addToken(TokenType::LeftParen),
            ')' => self.addToken(TokenType::RightParen),
            '{' => self.addToken(TokenType::LeftBrace),
            '}' => self.addToken(TokenType::RightBrace),
            ',' => self.addToken(TokenType::Comma),
            '.' => self.addToken(TokenType::Dot),
            '-' => self.addToken(TokenType::Minus),
            '+' => self.addToken(TokenType::Plus),
            ';' => self.addToken(TokenType::Semicolon),
            '*' => self.addToken(TokenType::Star),

            '!' => {
                let tokenType = if self.matchChar('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.addToken(tokenType);
            }
            '=' => {
                let tokenType = if self.matchChar('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.addToken(tokenType);
            }
            '>' => {
                let tokenType = if self.matchChar('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.addToken(tokenType);
            }
            '<' => {
                let tokenType = if self.matchChar('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.addToken(tokenType);
            }
            '/' => {
                if self.matchChar('/') {
                    while self.peek() != '\n' && !self.isAtEnd() {
                        self.advance();
                    }
                } else {
                    self.addToken(TokenType::Slash);
                }
            }
            '"' => {
                self.string();
            }

            '\n' => {
                self.line += 1;
            }
            ' ' | '\r' | '\t' => {}

            c => {
                if self.isDigit(c) {
                    self.number();
                } else if self.isAlpha(c) {
                    self.identifier();
                } else {
                    crate::error(self.line, "Unexpected character.");
                };
            }
        }
    }
}

// #[test]
// fn identifier_test() {
//     use std::fs;
//     use std::path::Path;

//     let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
//         .join("test_lox")
//         .join("identifiers.lox");
//     let file_content = fs::read_to_string(&file_path).unwrap_or_else(
//         |e| panic!("Read file:{} failed. Path:{:?}",
//         e,
//         file_path,
//     ));
//     let mut contents = Vec::new();
//     for line in file_content.lines() {
//         let trimmed_line = line.trim();
//         if trimmed_line.starts_with("// expect:") {
//             let expect_content = trimmed_line.strip_prefix("// expect:").unwrap().trim();
//             contents.push(expect_content.to_string());
//         }
//     }
//     let mut scanner = Scanner::new(&file_content);
//     let tokens = scanner.scanTokens();
//     for (idx, (token, line)) in tokens.into_iter().zip(contents.into_iter()).enumerate() {
//         let s = format!("{token}");
//         assert_eq!(
//             s, line,
//             "Line {} match failed, output:{}, expected:{}",
//             idx + 1, s, line
//         )
//     } 
// }

