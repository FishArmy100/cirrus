use crate::{ast::{AssignStmt, ExpressionStmt, Statement}, lexing::token::TokenType};

use super::{expect_expression, is_expression_and, parse_expression, token_reader::TokenReader, ParserError, ParserResult};

pub fn expect_statement(reader: &mut TokenReader) -> ParserResult<Statement>
{
    if let Some(statement) = parse_statement(reader)?
    {
        Ok(statement)
    }
    else
    {
        Err(ParserError::ExpectedStatement(reader.current()))    
    }
}

pub fn parse_statement(reader: &mut TokenReader) -> ParserResult<Option<Statement>>
{
    if let Some(expr) = parse_expression_stmt(reader)?
    {
        Ok(Some(expr))
    }
    else if let Some(assign) = parse_assignment(reader)?
    {
        Ok(Some(assign))
    }
    else 
    {
        Ok(None)    
    }
}

pub fn parse_expression_stmt(reader: &mut TokenReader) -> ParserResult<Option<Statement>>
{
    if let Some(expression) = is_expression_and(reader, |r| r.current_is(&[TokenType::SemiColon]))
    {
        let semi_colon = reader.expect(TokenType::SemiColon)?;
        Ok(Some(Statement::Expression(ExpressionStmt {
            expression,
            semi_colon
        })))
    }
    else 
    {
        Ok(None)    
    }
}


pub fn parse_assignment(reader: &mut TokenReader) -> ParserResult<Option<Statement>>
{
    if let Some(seq) = reader.check_sequence(&[TokenType::Identifier, TokenType::Equal])
    {
        let identifier = seq[0].clone();
        let equal = seq[1].clone();
        let expression = expect_expression(reader, parse_expression)?;
        let semi_colon = reader.expect(TokenType::SemiColon)?;

        Ok(Some(Statement::Assign(AssignStmt {
            identifier,
            equal,
            expression,
            semi_colon
        })))
    }
    else 
    {
        Ok(None)    
    }
}