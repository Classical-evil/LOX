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
        use crate::TokenType::LiteralsValue::*;
        match &self.literal {
            String(s) => {write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, s)
}
            Number(s) => {write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, s)
}
            Boolean(s) => {write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, s)
}
            _ => {write!(f, "{:?} {} {:?}", self.ttype, self.lexeme, self.literal)
}
        }
    }
}
