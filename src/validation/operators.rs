use std::collections::HashMap;
use crate::lexing::token::TokenType;

use super::type_key::TypeKey;

pub struct GlobalOperators
{
    binary_ops: HashMap<BinaryOpType, Vec<BinaryOp>>
}

impl GlobalOperators
{
    pub fn new() -> Self 
    {
        Self 
        {
            binary_ops: HashMap::new()
        }
    }

    pub fn add_binary_op(&mut self, op: BinaryOp) -> bool
    {
        let ops = self.binary_ops.entry(op.op).or_default();

        if ops.iter().any(|o| o.is_equivalent(&op))
        {
            return false;
        }
        
        ops.push(op);
        true
    }

    pub fn evaluate_binary(&self, left: &TypeKey, right: &TypeKey, op: BinaryOpType) -> Option<TypeKey>
    {
        let Some(ops) = self.binary_ops.get(&op) else {
            return None;
        };

        ops.iter().find(|o| o.left.is_equivalent(&left) && o.right.is_equivalent(&right)).map(|o| o.result.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOpType
{
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    And,
    Or,
}

impl BinaryOpType
{
    pub fn from_token_type(tt: TokenType) -> Option<BinaryOpType>
    {
        match tt 
        {
            TokenType::Plus => Some(Self::Plus),
            TokenType::Minus => Some(Self::Minus),
            TokenType::Multiply => Some(Self::Multiply),
            TokenType::Divide => Some(Self::Divide),
            TokenType::Modulus => Some(Self::Modulus),
            TokenType::EqualEqual => Some(Self::Equal),
            TokenType::BangEqual => Some(Self::NotEqual),
            TokenType::GreaterThan => Some(Self::GreaterThan),
            TokenType::GreaterEqual => Some(Self::GreaterThanEqual),
            TokenType::LessThan => Some(Self::LessThan),
            TokenType::LessEqual => Some(Self::LessThanEqual),
            TokenType::AndAnd => Some(Self::And),
            TokenType::PipePipe => Some(Self::Or),
            _ => None
        }
    }
}

impl std::fmt::Display for BinaryOpType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            BinaryOpType::Plus => write!(f, "+"),
            BinaryOpType::Minus => write!(f, "-"),
            BinaryOpType::Multiply => write!(f, "*"),
            BinaryOpType::Divide => write!(f, "/"),
            BinaryOpType::Modulus => write!(f, "%"),
            BinaryOpType::Equal => write!(f, "=="),
            BinaryOpType::NotEqual => write!(f, "!="),
            BinaryOpType::GreaterThan => write!(f, ">"),
            BinaryOpType::LessThan => write!(f, "<"),
            BinaryOpType::GreaterThanEqual => write!(f, ">="),
            BinaryOpType::LessThanEqual => write!(f, "<="),
            BinaryOpType::And => write!(f, "&&"),
            BinaryOpType::Or => write!(f, "||"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryOp 
{
    pub left: TypeKey,
    pub right: TypeKey,
    pub result: TypeKey,
    pub op: BinaryOpType,
}

impl BinaryOp
{
    pub fn is_equivalent(&self, other: &Self) -> bool
    {
        self.op == other.op && self.left.is_equivalent(&other.left) && self.right.is_equivalent(&other.right)
    }

    pub fn make_uniform(key: TypeKey, op: BinaryOpType) -> Self 
    {
        Self { left: key.clone(), right: key.clone(), result: key.clone(), op }
    }

    pub fn make_isosceles(args: TypeKey, result: TypeKey, op: BinaryOpType) -> Self
    {
        Self { left: args.clone(), right: args.clone(), result, op }
    }
}