pub mod builtins;
pub mod operators;
pub mod type_key;
pub mod typed_expr;
pub mod pattern;
use std::collections::HashMap;

use operators::{BinaryOpType, GlobalOperators};
use type_key::TypeKey;
use uuid::Uuid;

use crate::{compiler::{CompilerError, CompilerResult}, lexing::token::Token, utils::TextPos};

#[derive(Debug, Clone)]
pub enum TypeError
{
    ExpressionTypeNotImplementedYet
    {
        pos: TextPos
    },
    NoBinaryOperator
    {
        token: Token,
        op: BinaryOpType,
        left: TypeKey,
        right: TypeKey,
    }
}

impl CompilerError for TypeError
{
    fn pos(&self) -> Option<TextPos> 
    {
        match self 
        {
            TypeError::ExpressionTypeNotImplementedYet { pos } => Some(*pos),
            TypeError::NoBinaryOperator { token, op: _, left: _, right: _ } => Some(token.pos),
        }
    }

    fn message(&self) -> String 
    {
        match self 
        {
            TypeError::ExpressionTypeNotImplementedYet { pos: _ } => "Expression type not implemented".into(),
            TypeError::NoBinaryOperator { token: _, op, left, right } => {
                format!("Binary operator `{}` does not exist for types `{}` and `{}`", op, left, right)
            },
        }
    }
}

pub type TypeResult<T> = Result<T, TypeError>;

impl<T> CompilerResult<T> for TypeResult<T>
{
    fn is_ok(&self) -> bool 
    {
        self.is_ok()
    }

    fn get_result(&self) -> Option<&T> 
    {
        self.as_ref().ok()
    }

    fn get_errors(&self) -> Vec<impl CompilerError> 
    {
        match self 
        {
            Ok(_) => vec![],
            Err(e) => vec![e.clone()],
        }
    }
}




pub struct TypeContext
{
    pub operators: GlobalOperators,
    pub types: HashMap<Uuid, TypeKey>,

    pub int_id: Uuid,
    pub float_id: Uuid,
    pub bool_id: Uuid,
}