use crate::TokenType::LiteralsValue;
use crate::TokenType::TokenType;

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: LiteralsValue,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralsValue, line: usize) -> Self {
        Token {
            ttype: token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
        // write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, self.literal)
    }
}
