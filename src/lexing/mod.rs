use char_reader::CharReader;
use keywords::KEYWORDS;
use token::{Token, TokenPos, TokenType, TokenValue};

pub mod keywords;
pub mod token;
pub mod char_reader;

#[derive(Debug, Clone)]
pub enum LexerError 
{
    UnknownToken
    {
        token: char,
        index: usize,
    },
    UnterminatedString
    {
        index: usize,
    }
}

impl std::fmt::Display for LexerError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            LexerError::UnknownToken { token, index: _ } => write!(f, "Unknown token `{}`", token),
            LexerError::UnterminatedString { index: _ } => write!(f, "Unterminated string"),
        }
    }
}

#[derive(Debug)]
pub struct LexerResult
{
    pub text: Vec<char>,
    pub tokens: Vec<Token>,
    pub errors: Vec<LexerError>,
}

pub fn lex_text(text: &str) -> LexerResult
{
    let Some(mut reader) = CharReader::new(text) else {
        return LexerResult {
            text: vec![],
            tokens: vec![Token { token_type: TokenType::EOF, pos: 0.into(), value: None }],
            errors: vec![]
        };
    };

    let mut tokens = vec![];
    let mut errors = vec![];

    while !reader.at_end()
    {
        if reader.read_spaces()
        {
            continue;
        }
        else if let Some(symbol) = check_symbol(&mut reader)
        {
            tokens.push(symbol);
        }
        else if let Some(identifier) = check_identifier(&mut reader)
        {
            tokens.push(identifier);
        }
        else if let Some(literal) = check_string_literal(&mut reader)
        {
            match literal
            {
                Ok(ok) => tokens.push(ok),
                Err(err) => errors.push(err),
            }
        }
        else if let Some(literal) = check_number_literal(&mut reader)
        {
            tokens.push(literal);
        }
        else 
        {
            errors.push(LexerError::UnknownToken { 
                token: reader.advance().unwrap(), 
                index: reader.index() 
            });
        }
    }

    tokens.push(Token {
        pos: reader.index().into(),
        token_type: TokenType::EOF,
        value: None,
    });

    LexerResult { text: reader.chars().to_vec(), tokens, errors }
}

pub fn check_symbol(reader: &mut CharReader) -> Option<Token>
{
    let Some(c) = reader.current() else { return None };

    match c 
    {
        '(' => Some(make_token(reader, 1, TokenType::OpenParen)),
        ')' => Some(make_token(reader, 1, TokenType::CloseParen)),
        '[' => Some(make_token(reader, 1, TokenType::OpenBracket)),
        ']' => Some(make_token(reader, 1, TokenType::CloseBracket)),
        '{' => Some(make_token(reader, 1, TokenType::OpenBrace)),
        '}' => Some(make_token(reader, 1, TokenType::CloseBrace)),

        '+' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::PlusEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Plus))
            }
        },
        '-' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::MinusEqual))
            }
            else if reader.peek(1).is_some_and(|c| c == '>')
            {
                Some(make_token(reader, 2, TokenType::ThinArrow))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Minus))
            }
        },
        '*' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::MultiplyEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Multiply))
            }
        },
        '/' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::DivideEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Divide))
            }
        },
        '%' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::ModulusEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Modulus))
            }
        },

        '=' => {
            if reader.peek(1).is_some_and(|c| c == '>')
            {
                Some(make_token(reader, 2, TokenType::ThickArrow))
            }
            else if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::EqualEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Equal))
            }
        },
        '!' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::BangEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Bang))
            }
        },
        '<' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::LessEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::LessThan))
            }
        },
        '>' => {
            if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::GreaterEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::GreaterThan))
            }
        },

        '.' => Some(make_token(reader, 1, TokenType::Dot)),
        ',' => Some(make_token(reader, 1, TokenType::Comma)),
        ';' => Some(make_token(reader, 1, TokenType::SemiColon)),
        ':' => Some(make_token(reader, 1, TokenType::Colon)),

        '&' => {
            if reader.peek(1).is_some_and(|c| c == '&')
            {
                Some(make_token(reader, 2, TokenType::AndAnd))
            }
            else if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::AndEqual))
            }
            else 
            {
                None  
            }
        },
        '|' => {
            if reader.peek(1).is_some_and(|c| c == '|')
            {
                Some(make_token(reader, 2, TokenType::PipePipe))
            }
            else if reader.peek(1).is_some_and(|c| c == '=')
            {
                Some(make_token(reader, 2, TokenType::OrEqual))
            }
            else 
            {
                Some(make_token(reader, 1, TokenType::Pipe))
            }
        },
        _ => None,
    }
}

pub fn check_identifier(reader: &mut CharReader) -> Option<Token>
{
    let begin = reader.index();
    let mut text = String::new();

    if !reader.current().is_some_and(|c| c.is_alphabetic() || c == '_')
    {
        return None;
    }

    while reader.current().is_some_and(|c| c.is_alphanumeric() || c == '_')
    {
        text.push(reader.advance().unwrap());
    }

    if text.len() > 0
    {
        let end = reader.index() - 1;
        let token_type = match KEYWORDS.get(&text)
        {
            Some(t) => *t,
            None => TokenType::Identifier,
        };

        let value = match token_type
        {
            TokenType::Identifier => Some(TokenValue::String(text)),
            _ => None,
        };

        Some(Token { pos: TokenPos { begin, end }, token_type, value })
    }
    else 
    {
        None    
    }
}

pub fn check_string_literal(reader: &mut CharReader) -> Option<Result<Token, LexerError>> 
{
    if !reader.current().is_some_and(|c| c == '\"') { return None };

    let begin = reader.index();
    let mut text = String::new();
    text.push(reader.advance().unwrap());
    while !reader.current_is(&['\"']) && !reader.at_end()
    {
        let c = reader.advance().unwrap();
        text.push(c);
        if c == '\\'
        {
            if let Some(c) = reader.advance() { text.push(c); }
        }
    }

    if reader.current().is_some()
    {
        let end = reader.index();
        text.push(reader.advance().unwrap());
        Some(Ok(Token { 
            pos: TokenPos { begin, end }, 
            token_type: TokenType::StringLiteral, 
            value: Some(TokenValue::String(text)) 
        }))
    }
    else 
    {
        Some(Err(LexerError::UnterminatedString { index: begin }))
    }
}

pub fn check_number_literal(reader: &mut CharReader) -> Option<Token>
{
    if !reader.current().is_some_and(|c| c.is_digit(10)) { return None; }

    let begin = reader.index();
    let mut number = String::new();

    while reader.current().is_some_and(|c| c.is_digit(10))
    {
        number.push(reader.advance().unwrap());
    }

    if reader.current_is(&['.']) && reader.peek(1).is_some_and(|c| c.is_digit(10))
    {
        number.push(reader.advance().unwrap());
        while reader.current().is_some_and(|c| c.is_digit(10))
        {
            number.push(reader.advance().unwrap());
        }

        Some(Token 
        {
            pos: TokenPos { begin, end: reader.index() - 1 },
            token_type: TokenType::FloatLiteral,
            value: Some(TokenValue::Float(number.parse().unwrap()))
        })   
    }
    else 
    {
        Some(Token 
        {
            pos: TokenPos { begin, end: reader.index() - 1 },
            token_type: TokenType::IntegerLiteral,
            value: Some(TokenValue::Int(number.parse().unwrap()))
        })
    }
}

pub fn make_token(reader: &mut CharReader, length: usize, token_type: TokenType) -> Token
{
    assert!(length != 0, "Length of a token cannot be 0");
    let begin = reader.index();
    let end = reader.index() + length - 1;

    for _ in 0..length
    {
        reader.advance().expect("Expected a character");
    }

    Token 
    { 
        pos: TokenPos {
            begin,
            end
        }, 
        token_type, 
        value: None 
    }
}