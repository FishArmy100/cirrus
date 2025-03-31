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
    Struct,
    True,
    Type,
    Use,
    Var,
    Where,
    While,

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
    Int(u64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Token 
{
    pub pos: TokenPos,
    pub token_type: TokenType,
    pub value: Option<TokenValue>
}

#[derive(Debug, Clone, Copy)]
pub struct TokenPos
{
    pub begin: usize,
    pub end: usize,
}

impl From<usize> for TokenPos
{
    fn from(value: usize) -> Self 
    {
        Self 
        {
            begin: value,
            end: value
        }
    }
}