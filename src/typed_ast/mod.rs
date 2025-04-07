pub mod builtins;

use std::collections::HashMap;

use uuid::Uuid;

pub enum TypeError
{
    NoGenerics
    {
        type_id: TypeRef,
    }
}

pub type TypeInfoResult = Result<TypeInfo, TypeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOpType
{
    Invert, // !
    Negate, // -
}

#[derive(Debug, Clone)]
pub struct UnaryOp 
{
    pub operated: TypeRef,
    pub result: TypeRef,
    pub op: UnaryOpType,
}

impl UnaryOp
{
    pub fn uniform(type_id: TypeRef, op: UnaryOpType) -> Self 
    {
        Self 
        {
            operated: type_id.clone(),
            op,
            result: type_id
        }
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

#[derive(Debug, Clone)]
pub struct BinaryOp 
{
    pub left: TypeRef,
    pub right: TypeRef,
    pub result: TypeRef,
    pub op: BinaryOpType,
}

impl BinaryOp
{
    pub fn uniform(type_id: TypeRef, op: BinaryOpType) -> Self
    {
        Self 
        {
            left: type_id.clone(),
            right: type_id.clone(),
            op,
            result: type_id
        }
    }

    pub fn isosceles(args: TypeRef, op: BinaryOpType, ret: TypeRef) -> Self 
    {
        Self 
        { 
            left: args.clone(), 
            right: args.clone(), 
            result: ret, 
            op
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexOp
{
    pub indexed: TypeRef,
    pub indexer: TypeRef,
    pub result: TypeRef,
}

#[derive(Debug, Clone)]
pub struct CallOp
{
    pub called: TypeInfo,
    pub args: Vec<TypeInfo>,
    pub result: TypeInfo,
}

#[derive(Debug, Clone)]
pub struct CastOp
{
    pub casted: TypeRef,
    pub caster: TypeRef,
}

#[derive(Debug, Clone)]
pub struct Member
{
    pub name: String,
    pub type_info: TypeRef,
    pub is_static: bool, // fn(self? ...)
    pub is_pub: bool,
    pub is_type_def: bool, // type T = Int
}

#[derive(Debug, Clone)]
pub struct Variant 
{
    pub name: String,
    pub type_info: TypeRef
}

#[derive(Debug, Clone)]
pub struct TypeInfo 
{
    pub id: TypeRef,
    pub name: String,

    pub unary_ops: HashMap<UnaryOpType, UnaryOp>,
    pub binary_ops: HashMap<BinaryOpType, BinaryOp>,
    pub index_op: Option<IndexOp>,
    pub call_ops: Vec<CallOp>,
    pub members: Vec<Member>, // functions, types, 
    pub variants: Vec<Variant>,
    pub cast_ops: Vec<CastOp>,

    pub is_variant: bool,
}

impl TypeInfo
{
    pub fn new_primary(id: TypeRef, name: impl ToString) -> Self
    {
        Self 
        {
            id,
            name: name.to_string(),
            unary_ops: HashMap::new(),
            binary_ops: HashMap::new(),
            index_op: None,
            call_ops: vec![],
            members: vec![],
            variants: vec![],
            cast_ops: vec![],
            is_variant: false,
        }
    }

    pub fn insert_uniform_binary_ops(&mut self, ops: &[BinaryOpType]) -> &mut Self
    {
        ops.iter().for_each(|op| {
            self.binary_ops.insert(*op, BinaryOp::uniform(self.id.clone(), *op));
        });
        self
    }

    pub fn insert_uniform_unary(&mut self, op: UnaryOpType) -> &mut Self
    {
        self.unary_ops.insert(op, UnaryOp::uniform(self.id.clone(), op));
        self
    }

    pub fn insert_casts(&mut self, casters: &[TypeRef]) -> &mut Self
    {
        casters.iter().for_each(|c| {
            self.cast_ops.push(CastOp { casted: self.id.clone(), caster: c.clone() });
        });

        self
    }

    pub fn insert_isosceles_binary_ops(&mut self, ops: &[BinaryOpType], ret: TypeRef) -> &mut Self 
    {
        ops.iter().for_each(|op| {
            self.binary_ops.insert(*op, BinaryOp::isosceles(self.id.clone(), *op, ret));
        });

        self
    }
}

pub type TypeRef = Uuid;

pub struct TypeGenerator(Box<dyn Fn(Vec<TypeInfo>) -> TypeInfoResult>);

impl TypeGenerator
{
    pub fn gen(&self, types: Vec<TypeInfo>) -> TypeInfoResult
    {
        self.0(types)
    }
}



pub struct TypedAst
{
    pub types: HashMap<Uuid, TypeGenerator>
}