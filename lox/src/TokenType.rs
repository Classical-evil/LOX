#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    //Single-character tokens
    // LEFT_PAREN,
    // RIGHT_PAREN,
    // LEFT_BRACE,
    // RIGHT_BRACE,
    // COMMA,
    // DOT,
    // MINUS,
    // PLUS,
    // SEMICOLON,
    // SLASH,
    // STAR,

    // //One or teo character tokens
    // BANG,
    // BANG_EQUAL,
    // EQUAL,
    // EQUAL_EQUAL,
    // GREATER,
    // GREATER_EQUAL,
    // LESS,
    // LESS_EQUAL,

    // //Literals
    // IDENTIFIER,
    // STRING,
    // NUMBER,

    // //keywords
    // AND,
    // CLASS,
    // ELSE,
    // FALSE,
    // FUN,
    // FOR,
    // IF,
    // NIL,
    // OR,
    // PRINT,
    // RETURN,
    // SUPER,
    // THIS,
    // TRUE,
    // VAR,
    // WHILE,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,
    // 新增：双字符运算符相关
    Bang,         // 对应Java的BANG (!)
    BangEqual,    // 对应Java的BANG_EQUAL (!=)
    Equal,        // 对应Java的EQUAL (=)
    EqualEqual,   // 对应Java的EQUAL_EQUAL (==)
    Less,         // 对应Java的LESS (<)
    LessEqual,    // 对应Java的LESS_EQUAL (<=)
    Greater,      // 对应Java的GREATER (>)
    GreaterEqual, // 对应Java的GREATER_EQUAL (>=)
    // 其他已定义变体
    Eof,
    Identifier,
    String,
    Number,
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // 静态不可变HashMap：&str -> TokenType
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        // 填充Lox所有关键字（按实际需求补充）
        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);
        map
    };
}

#[derive(Debug)]
pub enum LiteralsValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
