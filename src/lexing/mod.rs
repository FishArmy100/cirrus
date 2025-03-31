use char_reader::CharReader;
use token::{Token, TokenPos, TokenType};

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
    }
}

#[derive(Debug)]
pub enum LexerResult
{
    Empty,
    Error 
    {
        text: Vec<char>,
        errors: Vec<LexerError>,
    },
    Success
    {
        text: Vec<char>,
        tokens: Vec<Token>,
    }
}

pub fn lex_text(text: &str) -> LexerResult
{
    let Some(mut reader) = CharReader::new(text) else {
        return LexerResult::Empty;
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
        else 
        {
            errors.push(LexerError::UnknownToken { 
                token: reader.current().unwrap(), 
                index: reader.index() 
            });
        }
    }

    if errors.len() > 0
    {
        return LexerResult::Error { text: reader.chars().to_vec(), errors }
    }

    tokens.push(Token {
        pos: reader.index().into(),
        token_type: TokenType::EOF,
        value: None,
    });

    LexerResult::Success { text: reader.chars().to_vec(), tokens }
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