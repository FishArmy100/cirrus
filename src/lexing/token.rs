use crate::utils::{TextLocation, TextPos};


pub const ASSIGNMENT_TOKENS: &'static [TokenType] = &[
    TokenType::Equal,
    TokenType::PlusEqual,
    TokenType::MinusEqual,
    TokenType::MultiplyEqual,
    TokenType::DivideEqual,
    TokenType::ModulusEqual,
    TokenType::AndEqual,
    TokenType::OrEqual,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType
{
    // Keywords
    As,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    False,
    Fn,
    For,
    If,
    Impl,
    In,
    Interface,
    Let,
    Match,
    Mod,
    Return,
    Pub,
    SelfVal,
    SelfType,
    Struct,
    True,
    Type,
    Use,
    Mut,
    Where,
    While,
    Yield,

    // Tokens
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,

    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,

    PlusEqual,
    MinusEqual,
    MultiplyEqual,
    DivideEqual,
    ModulusEqual,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    LessEqual,
    GreaterEqual,
    GreaterThan,
    LessThan,
    OrEqual,
    AndEqual,

    ThinArrow,
    ThickArrow,

    Dot,
    AndAnd,
    PipePipe,
    Pipe,
    SemiColon,
    Colon,
    Comma,

    // Literal
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,

    // Identifier
    Identifier,

    // End of file
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue
{
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl TokenValue
{
    pub fn as_string(&self) -> Option<&String>
    {
        match self 
        {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64>
    {
        match self 
        {
            Self::Int(i) => Some(*i),
            _ => None
        }
    }
}

impl std::fmt::Display for TokenValue
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            TokenValue::String(s) => write!(f, "{}", s),
            TokenValue::Int(i) => write!(f, "{}", i),
            TokenValue::Float(n) => write!(f, "{}", n),
            TokenValue::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token 
{
    pub pos: TextPos,
    pub token_type: TokenType,
    pub value: Option<TokenValue>
}

impl Token 
{
    pub fn value_string(&self) -> Option<&String>
    {
        self.value.as_ref().map(|s| s.as_string()).flatten()
    }

    pub fn value_int(&self) -> Option<i64>
    {
        self.value.as_ref().map(|s| s.as_int()).flatten()
    }

    pub fn get_loc(&self, text: &[char]) -> TextLocation
    {
        self.pos.get_loc(text)
    }
}

impl std::fmt::Display for Token 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match &self.value
        {
            Some(TokenValue::Float(v)) => write!(f, "{:?}({})", self.token_type, v),
            Some(TokenValue::Int(v)) => write!(f, "{:?}({})", self.token_type, v),
            Some(TokenValue::String(v)) => write!(f, "{:?}({})", self.token_type, v),
            Some(TokenValue::Bool(v)) => write!(f, "{:?}({})", self.token_type, v),
            None => write!(f, "{:?}", self.token_type)
        }
    }
}