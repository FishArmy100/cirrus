use std::collections::HashMap;

use itertools::Itertools;
use uuid::Uuid;

use super::{GenericParam, InterfaceDef, StructDef};


pub const INT_TYPE_NAME: &str = "Int";
pub const BOOL_TYPE_NAME: &str = "Bool";
pub const FLOAT_TYPE_NAME: &str = "Float";

pub const ADD_INTERFACE_NAME: &str = "Add";

#[derive(Debug, Clone)]
pub struct Builtins 
{
    pub int_id: Uuid,
    pub add_id: Uuid,
}

#[derive(Debug)]
pub struct BuiltinsResult
{
    pub structs: HashMap<Uuid, StructDef>,
    pub interfaces: HashMap<Uuid, InterfaceDef>,
    pub builtins: Builtins,
}

pub fn get_builtin_types() -> BuiltinsResult
{
    let mut structs = HashMap::new();
    let int_id = append_builtin_type(&mut structs, INT_TYPE_NAME, vec![], StructDef::new_builtin);

    let mut interfaces = HashMap::new();
    let add_id = append_builtin_type(&mut interfaces, ADD_INTERFACE_NAME, vec!["Result"], InterfaceDef::new_builtin);

    let builtins = Builtins {
        int_id,
        add_id,
    };

    BuiltinsResult {
        structs,
        interfaces,
        builtins,
    }
}

fn append_builtin_type<B, F>(map: &mut HashMap<Uuid, B>, name: &str, params: Vec<&str>, f: F) -> Uuid
    where F : Fn(&str, Vec<GenericParam>) -> B,
          B : IdAble
{
    let b = f(name, params.iter().map(|b| GenericParam { name: b.to_string() }).collect_vec());
    let id = b.get_id();
    map.insert(id.clone(), b);
    id
}

trait IdAble
{
    fn get_id(&self) -> Uuid;
}

impl IdAble for StructDef
{
    fn get_id(&self) -> Uuid 
    {
        self.id.clone()
    }
}

impl IdAble for InterfaceDef
{
    fn get_id(&self) -> Uuid 
    {
        self.id.clone()
    }
}