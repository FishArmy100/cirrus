use either::Either;

use crate::lexing::token::Token;

use super::{*, stmt::*};

#[derive(Debug, Clone)]
pub struct LambdaParam
{
    pub name: Token,
    pub colon: Option<Token>,
    pub type_name: Option<TypeName>
}

#[derive(Debug, Clone)]
pub struct LambdaParams
{
    pub open_pipe: Token,
    pub parameters: Vec<LambdaParam>,
    pub close_pipe: Token,
    pub arrow: Option<Token>,
    pub return_type: Option<TypeName>,
}

#[derive(Debug, Clone)]
pub struct LambdaExpr
{
    pub params: LambdaParams,
    pub arrow: Token,
    pub expression: Box<Expression>
}

#[derive(Debug, Clone)]
pub struct CallExpr
{
    pub expression: Box<Expression>,
    pub open_paren: Token,
    pub args: Vec<Expression>,
    pub close_paren: Token,
}

#[derive(Debug, Clone)]
pub struct IndexExpr
{
    pub expression: Box<Expression>,
    pub open_bracket: Token,
    pub indexer: Box<Expression>,
    pub close_bracket: Token,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr
{
    pub open_paren: Token,
    pub expression: Box<Expression>,
    pub close_paren: Token,
}

#[derive(Debug, Clone)]
pub struct Access
{
    pub expression: Box<Expression>,
    pub dot: Token,
    pub identifier: Token,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr
{
    pub expression: Box<Expression>,
    pub operator: Token,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr
{
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct BlockExpr
{
    pub statements: Vec<Statement>,
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct ConstructionArg
{
    pub name: Token,
    pub colon: Token,
    pub value: Box<Expression>
}

#[derive(Debug, Clone)]
pub struct ConstructionExpr
{
    pub type_name: TypeName,
    pub open_brace: Token,
    pub args: Vec<ConstructionArg>,
    pub close_brace: Token,
}

#[derive(Debug, Clone)]
pub struct TypeValueExpr
{
    pub type_name: TypeName,
    pub dot: Token,
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct MatchBranch
{
    pub pattern: Pattern,
    pub arrow: Token,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct MatchExpr
{
    pub match_tok: Token,
    pub expression: Box<Expression>,
    pub open_brace: Token,
    pub branches: Vec<MatchBranch>,
    pub close_brace: Token,
}

#[derive(Debug, Clone)]
pub enum IfCond
{
    Expression(Box<Expression>),
    Pattern 
    {
        let_tok: Token,
        pattern: Pattern,
        equal: Token,
        expression: Box<Expression>,
        and: Option<Token>,
        other_cond: Box<IfCond>,
    }
}

#[derive(Debug, Clone)]
pub struct IfExpr
{
    pub if_tok: Token,
    pub condition: IfCond,
    pub block: BlockExpr,
    pub else_branch: Option<ElseBranch>
}

#[derive(Debug, Clone)]
pub struct ElseBranch
{
    pub else_tok: Token,
    pub body: Either<Box<IfExpr>, BlockExpr>,
}

#[derive(Debug, Clone)]
pub enum Expression
{
    Lambda(LambdaExpr),
    Literal(Token),
    Identifier(Token),
    Grouping(GroupingExpr),
    SelfExpr(Token),
    BlockExpr(BlockExpr),
    TypeValue(TypeValueExpr),
    Construction(ConstructionExpr),
    Call(CallExpr),
    Index(IndexExpr),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    IfExpr(IfExpr),
    MatchExpr(MatchExpr),
}