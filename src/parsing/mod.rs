pub mod token_reader;
pub mod type_parsing;
pub mod expr_parsing;
pub mod stmt_parsing;
pub mod pattern_parsing;

use pattern_parsing::expect_pattern;
pub use type_parsing::*;
pub use expr_parsing::*;

use token_reader::TokenReader;

use crate::lexing::token::{Token, TokenType};
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
}

pub type ParserResult<T> = Result<T, ParserError>;

pub fn parse(tokens: Vec<Token>) -> ParserResult<Option<Expression>>
{
    let Some(mut reader) = TokenReader::new(&tokens, None) else { return Ok(None) };
    parse_expression(&mut reader)
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

pub fn parse_generic_args(reader: &mut TokenReader) -> ParserResult<Option<GenericArgs>>
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

pub fn parse_generic_params(reader: &mut TokenReader) -> ParserResult<Option<GenericParams>>
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

pub fn expect_ast_item<P, R, E>(reader: &mut TokenReader, predicate: P, error: E) -> ParserResult<R>
    where P : Fn(&mut TokenReader) -> ParserResult<Option<R>>,
          E : Fn(Option<Token>) -> ParserError
{
    match predicate(reader)?
    {
        Some(r) => Ok(r),
        None => Err(error(reader.current()))
    }
}

