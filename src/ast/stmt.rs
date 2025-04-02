use crate::lexing::token::Token;

use super::Expression;


#[derive(Debug, Clone)]
pub struct ExpressionStmt
{
    pub expression: Expression,
    pub semi_colon: Token,
}

#[derive(Debug, Clone)]
pub struct AssignStmt
{
    pub identifier: Token,
    pub equal: Token,
    pub expression: Expression,
    pub semi_colon: Token,
}

#[derive(Debug, Clone)]
pub enum Statement
{
    Expression(ExpressionStmt),
    Assign(AssignStmt),
}