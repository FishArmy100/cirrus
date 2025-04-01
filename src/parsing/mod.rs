pub mod token_reader;

use token_reader::TokenReader;

use crate::{ast::{expr::{Expression, GroupingExpr, LambdaExpr, LambdaParam, LambdaParams}, TypeName}, lexing::token::{Token, TokenType}};

pub enum ParserError
{
    ExpectedExpression(Option<Token>),
    ExpectedType(Option<Token>),
    ExpectedToken(TokenType),
    ExpectedALambdaParameter(Option<Token>)
}

pub type ParserResult<T> = Result<T, ParserError>;

pub fn parse(tokens: Vec<Token>) -> ParserResult<Option<Expression>>
{
    let Some(mut reader) = TokenReader::new(tokens) else { return Ok(None) };
    parse_expression(&mut reader)
}

pub fn parse_expression(reader: &mut TokenReader) -> ParserResult<Option<Expression>>
{
    todo!()
}

pub fn parse_primary(reader: &mut TokenReader) -> ParserResult<Option<Expression>>
{
    if let Some(literal) = reader.check_many(&[
        TokenType::IntegerLiteral,
        TokenType::StringLiteral,
        TokenType::FloatLiteral,
        TokenType::Identifier,
        TokenType::SelfVal,
        TokenType::True,
        TokenType::False,
    ])
    {
        Ok(Some(Expression::Literal(literal)))
    }
    else 
    {
        Ok(None)
    }
}

pub fn parse_grouping(reader: &mut TokenReader) -> ParserResult<Option<Expression>>
{
    if let Some(open_paren) = reader.check(TokenType::OpenParen)
    {
        let Some(expression) = parse_expression(reader)? else {
            return Err(ParserError::ExpectedExpression(reader.current()))
        };
        let expression = Box::new(expression);

        let close_paren = reader.expect(TokenType::CloseParen)?;
        let grouping = GroupingExpr { open_paren, expression, close_paren };
        Ok(Some(Expression::Grouping(grouping)))
    }
    else 
    {
        Ok(None)
    }
}

pub fn parse_lambda_param(reader: &mut TokenReader) -> ParserResult<Option<LambdaParam>>
{
    let Some(name) = reader.check(TokenType::Identifier) else {
        return Ok(None)
    };

    if let Some(colon) = reader.check(TokenType::Colon)
    {
        let Some(type_name) = parse_type_name(reader)? else {
            return Err(ParserError::ExpectedType(reader.current()))
        };
        Ok(Some(LambdaParam { 
            name, 
            colon: Some(colon), 
            type_name: Some(type_name) 
        }))
    }
    else 
    {
        Ok(Some(LambdaParam { 
            name, 
            colon: None, 
            type_name: None 
        }))
    }
}

pub fn parse_lambda_params(reader: &mut TokenReader) -> ParserResult<LambdaParams>
{
    let mut parameters = vec![];
    while reader.check(TokenType::Pipe).is_none()
    {
        let Some(param) = parse_lambda_param(reader)? else {
            return Err(ParserError::ExpectedALambdaParameter(reader.current()));
        };

        parameters.push(param);

        if !reader.current_is(&[TokenType::Pipe, TokenType::Comma])
        {
            return Err(ParserError::ExpectedToken(TokenType::Pipe));
        }
    }
}

pub fn parse_lambda(reader: &mut TokenReader) -> ParserResult<Option<Expression>>
{
    if let Some(open_pipe) = reader.check(TokenType::Pipe)
    {
        

        let close_pipe = reader.previous().unwrap();

        let arrow = reader.expect(TokenType::ThickArrow)?;
        let Some(expression) = parse_expression(reader)? else {
            return Err(ParserError::ExpectedExpression(reader.current()));
        }

        Ok(Some(Expression::Lambda(LambdaExpr {
            params,
            arrow,
            expression
        })))
    }
    else 
    {
        Ok(None)    
    }
}

pub fn parse_type_name(reader: &mut TokenReader) -> ParserResult<Option<TypeName>>
{
    todo!()
}
