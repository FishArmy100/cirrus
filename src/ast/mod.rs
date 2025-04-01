pub mod expr;
pub mod stmt;
pub use expr::*;
use itertools::Itertools;
pub use stmt::*;

use crate::lexing::token::Token;

#[derive(Debug, Clone)]
pub enum TypeName
{
    Identifier(Token, Option<GenericArgs>),
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
        close_paren: Token,
        arrow: Token,
        return_type: Box<TypeName>,
    }
}

impl TypeName
{
    pub fn pretty_print(&self) -> String 
    {
        match self
        {
            TypeName::Identifier(token, generic_args) =>
            {
                let mut text = token.value.as_ref().unwrap().to_string();
                if let Some(args) = generic_args
                {
                    text += &format!("[{}]", args.args.iter().map(|a| a.pretty_print()).join(", "));
                }

                text
            },
            TypeName::Array { open_bracket: _, close_bracket: _, type_name } => 
            {
                format!("[]{}", type_name.pretty_print())
            },
            TypeName::Function { fn_tok: _, open_paren: _, parameter_types, close_paren: _, arrow: _, return_type } => 
            {
                format!("fn({}) -> {}", parameter_types.iter().map(|t| t.pretty_print()).join(", "), return_type.pretty_print())
            },
        }
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