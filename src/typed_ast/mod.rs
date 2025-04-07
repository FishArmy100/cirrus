use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOpType
{
    Invert, // !
    Negate, // -
}

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
}

pub struct IndexOp
{
    pub indexed: TypeRef,
    pub indexer: TypeRef,
    pub result: TypeRef,
}

pub struct CallOp
{
    pub called: TypeInfo,
    pub args: Vec<TypeInfo>,
    pub result: TypeInfo,
}

pub struct Member
{
    pub name: String,
    pub type_info: TypeRef,
    pub is_static: bool, // fn(self? ...)
    pub is_pub: bool,
    pub is_type_def: bool, // type T = Int
}

pub struct Variant 
{
    pub name: String,
    pub type_info: TypeRef
}

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

    pub is_variant: bool,
}

impl TypeInfo
{
    pub fn new_primary(name: impl ToString) -> Self
    {
        Self 
        {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            unary_ops: HashMap::new(),
            binary_ops: HashMap::new(),
            index_op: None,
            call_ops: vec![],
            members: vec![],
            variants: vec![],
            is_variant: false,
        }
    }

    pub fn insert_uniform_binary_ops(&mut self, ops: &[BinaryOpType]) -> &mut Self
    {
        self
    }

    pub fn insert_uniform_unary_ops(&mut self, ops: &[UnaryOpType]) -> &mut Self
    {
        self
    }
}

pub type TypeRef = Uuid;

pub struct TypeGenerator(Box<dyn Fn(Vec<TypeInfo>) -> TypeInfo>);

impl TypeGenerator
{
    pub fn gen(&self, types: Vec<TypeInfo>) -> TypeInfo
    {
        self.0(types)
    }
}

fn build_builtins() -> HashMap<Uuid, TypeGenerator>
{
    let mut map = HashMap::new();

    let mut int_type = TypeInfo::new_primary("Int");
    let int_id = int_type.id.clone();

    int_type.binary_ops.insert(BinaryOpType::Plus, BinaryOp { left: int_id.clone(), right: (), result: (), op: () })

    map
}

pub struct TypedAst
{
    pub types: HashMap<Uuid, TypeGenerator>
}