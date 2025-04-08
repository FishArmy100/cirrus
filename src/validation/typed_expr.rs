use crate::{ast::{BinaryExpr, Expression}, lexing::token::TokenValue};

use super::{operators::BinaryOpType, type_key::TypeKey, TypeContext, TypeError};

#[derive(Debug)]
pub enum TypedExpr 
{
    Binary 
    {
        right: Box<TypedExpr>,
        op: BinaryOpType,
        left: Box<TypedExpr>,
        ret: TypeKey,
    },
    FloatLiteral(f64),
    IntLiteral(i64),
    BoolLiteral(bool),
}

impl TypedExpr
{
    fn get_type(&self, context: &TypeContext) -> TypeKey
    {
        match self 
        {
            TypedExpr::Binary { right: _, op: _, left: _, ret } => ret.clone(),
            TypedExpr::FloatLiteral(_) => context.types.get(&context.float_id).unwrap().clone(),
            TypedExpr::IntLiteral(_) => context.types.get(&context.int_id).unwrap().clone(),
            TypedExpr::BoolLiteral(_) => context.types.get(&context.bool_id).unwrap().clone(),
        }
    }

    pub fn from_ast(context: &TypeContext, ast: &Expression) -> Result<Self, TypeError>
    {
        match ast
        {
            Expression::Binary(BinaryExpr { left, operator, right }) => 
            {
                let left = TypedExpr::from_ast(context, left)?;
                let left_type = left.get_type(context);

                let right = TypedExpr::from_ast(context, right)?;
                let right_type = right.get_type(context);

                let op = BinaryOpType::from_token_type(operator.token_type).expect("Unknown operator token");

                match context.operators.evaluate_binary(&left_type, &right_type, op)
                {
                    Some(ret) => Ok(TypedExpr::Binary { 
                        right: Box::new(right), 
                        op, left: 
                        Box::new(left), 
                        ret 
                    }),
                    None => Err(TypeError::NoBinaryOperator { 
                        op, 
                        left: left_type, 
                        right: right_type, 
                        token: operator.clone() 
                    })
                }
            }

            Expression::Literal(token) => 
            {
                match token.value {
                    Some(TokenValue::Int(v)) => Ok(TypedExpr::IntLiteral(v)),
                    Some(TokenValue::Float(v)) => Ok(TypedExpr::FloatLiteral(v)),
                    Some(TokenValue::Bool(v)) => Ok(TypedExpr::BoolLiteral(v)),
                    _ => panic!("Internal compiler error: Unknown literal type")
                }
            }

            e => Err(TypeError::ExpressionTypeNotImplementedYet { pos: e.get_pos() })
        }
    }
}