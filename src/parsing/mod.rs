pub mod token_reader;
pub mod type_parsing;
pub mod expr_parsing;
pub mod stmt_parsing;
pub mod pattern_parsing;

use pattern_parsing::expect_pattern;
use stmt_parsing::parse_declaration;
pub use type_parsing::*;
pub use expr_parsing::*;

use token_reader::TokenReader;

use crate::lexing::token::{Token, TokenTextLocation, TokenType};
use crate::ast::*;

#[derive(Debug)]
pub enum ParserError
{
    ExpectedExpression(Option<Token>),
    ExpectedType(Option<Token>),
    ExpectedToken(TokenType, Option<Token>),
    ExpectedTokens(Vec<TokenType>, Option<Token>),
    ExpectedALambdaParameter(Option<Token>),
    ExpectedStatement(Option<Token>),
    ExpectedPattern(Option<Token>),
    ExpectedBlock(Option<Token>),
    ExpectedDeclaration(Option<Token>),
}

impl ParserError
{
    pub fn format(&self, text: &[char], file: &str) -> String 
    {
        let line_count = text.iter().filter(|f| **f == '\n').count() + 1;
        let end_loc = TokenTextLocation { line: line_count, column: 1 };
        
        let formatter = |token: &Option<Token>, error: &str| { 
            format!("[{}:{}]: {}", file, token.as_ref().map_or(end_loc, |t| t.get_loc(text)), error)
        };

        match self
        {
            ParserError::ExpectedExpression(token) => formatter(token, "Expected an expression"),
            ParserError::ExpectedType(token) => formatter(token, "Expected a type"),
            ParserError::ExpectedToken(token_type, token) => formatter(token, &format!("Expected token {:?} ", token_type)),
            ParserError::ExpectedTokens(token_types, token) => formatter(token, &format!("Expected one of token {:?} ", token_types.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>())),
            ParserError::ExpectedALambdaParameter(token) => formatter(token, "Expected a lambda parameter"),
            ParserError::ExpectedStatement(token) => formatter(token, "Expected a statement"),
            ParserError::ExpectedPattern(token) => formatter(token, "Expected a pattern"),
            ParserError::ExpectedBlock(token) => formatter(token, "Expected a block expression"),
            ParserError::ExpectedDeclaration(token) => formatter(token, "Expected a declaration"),
        }
    }
}

pub type ParserResult<T> = Result<T, ParserError>;

pub fn parse(tokens: Vec<Token>) -> ParserResult<Option<Program>>
{
    let Some(mut reader) = TokenReader::new(&tokens, None) else { return Ok(None) };
    let mut declarations = vec![];
    while let Some(declaration) = parse_declaration(&mut reader)?
    {
        declarations.push(declaration);
    }

    let eof = reader.expect(TokenType::EOF)?;

    Ok(Some(Program { declarations, eof }))
}

fn expect_let_condition(reader: &mut TokenReader) -> ParserResult<LetCondition>
{
    match parse_let_condition(reader)?
    {
        Some(ok) => Ok(ok),
        None => Err(ParserError::ExpectedExpression(reader.current()))
    }
}

fn parse_let_condition(reader: &mut TokenReader) -> ParserResult<Option<LetCondition>>
{
    if let Some(let_tok) = reader.check(TokenType::Let)
    {
        let pattern = expect_pattern(reader)?;
        let equal = reader.expect(TokenType::Equal)?;
        let expression = expect_expression(reader, parse_expression)?;

        if let Some(and) = reader.check(TokenType::AndAnd)
        {
            let Some(other_cond) = parse_let_condition(reader)? else {
                return Err(ParserError::ExpectedExpression(reader.current()));
            };

            Ok(Some(LetCondition::Pattern { 
                let_tok, 
                pattern, 
                equal, 
                expression: Box::new(expression), 
                and: Some(and), 
                other_cond: Some(Box::new(other_cond))
            }))
        }
        else 
        {
            Ok(Some(LetCondition::Pattern { 
                let_tok, 
                pattern, 
                equal, 
                expression: Box::new(expression), 
                and: None, 
                other_cond: None,
            }))  
        }
    }
    else 
    {
        let Some(expression) = parse_expression(reader)? else {
            return Ok(None);
        };

        Ok(Some(LetCondition::Expression(Box::new(expression))))
    }
}

fn parse_generic_args(reader: &mut TokenReader) -> ParserResult<Option<GenericArgs>>
{
    let Some(open_bracket) = reader.check(TokenType::OpenBracket) else {
        return Ok(None);
    };

    let mut types = vec![];
    while !reader.current_is(&[TokenType::CloseBracket])
    {
        let Some(type_name) = parse_type_name(reader)? else {
            return Err(ParserError::ExpectedType(reader.current()));
        };

        types.push(type_name);

        if !reader.current_is(&[TokenType::CloseBracket, TokenType::Comma])
        {
            return Err(ParserError::ExpectedToken(TokenType::CloseBracket, reader.current()));
        }

        let _ = reader.check(TokenType::Comma); // makes sure to skip the comma
    }

    let close_bracket = reader.expect(TokenType::CloseBracket)?;
    let args = GenericArgs {
        open_bracket,
        args: types,
        close_bracket
    };

    Ok(Some(args))
}

fn parse_generic_params(reader: &mut TokenReader) -> ParserResult<Option<GenericParams>>
{
    let Some(open_bracket) = reader.check(TokenType::OpenBracket) else {
        return Ok(None)
    };

    let mut params = vec![];
    while let Some(id) = reader.check(TokenType::Identifier)
    {
        params.push(id);
        if !reader.check(TokenType::Comma).is_some() { break; }
    }

    let close_bracket = reader.expect(TokenType::CloseBracket)?;
    Ok(Some(GenericParams { open_bracket, params, close_bracket }))
}

fn expect_ast_item<P, R, E>(reader: &mut TokenReader, predicate: P, error: E) -> ParserResult<R>
    where P : Fn(&mut TokenReader) -> ParserResult<Option<R>>,
          E : Fn(Option<Token>) -> ParserError
{
    match predicate(reader)?
    {
        Some(r) => Ok(r),
        None => Err(error(reader.current()))
    }
}

