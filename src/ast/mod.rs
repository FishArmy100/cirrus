pub mod expr;
pub mod stmt;
use expr::Expression;

use crate::lexing::token::Token;

#[derive(Debug, Clone)]
pub enum TypeName
{
    Identifier(Vec<(Token, Option<GenericArgs>)>),
    Array
    {
        open_bracket: Token,
        close_bracket: Token,
        type_name: Box<TypeName>,
    },
    Function 
    {
        fn_tok: Token,
        open_paren: Token,
        parameter_types: Vec<TypeName>,
        close_token: Token,
        return_arrow: Option<Token>,
        return_type: Option<Box<TypeName>>,
    }
}

#[derive(Debug, Clone)]
pub struct GenericParams
{
    pub open_bracket: Token,
    pub params: Vec<Token>,
    pub close_bracket: Token,
}

#[derive(Debug, Clone)]
pub struct GenericArgs 
{
    pub open_bracket: Token,
    pub args: Vec<TypeName>,
    pub close_bracket: Token,
}

#[derive(Debug, Clone)]
pub struct Parameters
{
    pub open_paren: Token,
    pub parameters: Vec<Parameter>,
    pub close_paren: Token,
}

#[derive(Debug, Clone)]
pub struct Parameter 
{
    pub var: Option<Token>,
    pub name: Token,
    pub colon: Token,
    pub type_name: TypeName,
    pub equal: Option<Token>,
    pub expression: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct PatternField
{
    pub name: Token,
    pub colon: Token,
    pub pattern: Pattern,
}

#[derive(Debug, Clone)]
pub enum Pattern 
{
    Number(Token),
    String(Token),
    Identifier(Vec<Token>),
    EnumPattern
    {
        id_list: Vec<Token>,
        open_paren: Token,
        pattern: Box<Pattern>,
        close_paren: Token,
    },
    StructPattern
    {
        id_list: Vec<Token>,
        open_brace: Token,
        patterns: Vec<PatternField>,
        close_brace: Token,
    },
}