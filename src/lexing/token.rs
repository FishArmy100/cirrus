
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
    Int(u64),
    Float(f64),
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token 
{
    pub pos: TokenPos,
    pub token_type: TokenType,
    pub value: Option<TokenValue>
}

impl Token 
{
    pub fn get_loc(&self, text: &[char]) -> TokenTextLocation
    {
        let mut line = 1;
        let mut column = 1;

        for i in 0..=self.pos.begin
        {
            if text[i] == '\n'
            {
                line += 1;
                column = 0;
            }
            else 
            {
                column += 1;
            }
        }

        TokenTextLocation { line, column }
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
            None => write!(f, "{:?}", self.token_type)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenTextLocation
{
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for TokenTextLocation
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}:{}", self.line, self.column)
    }
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