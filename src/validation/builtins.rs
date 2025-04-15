use std::collections::HashMap;

use itertools::Itertools;
use uuid::Uuid;

use super::{GenericParam, InterfaceDef, InterfaceImpl, ProgramTypeDefinitions, StructDef, TypeContext, TypePattern};


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

pub fn get_builtin_impls(builtins: &Builtins) -> Vec<InterfaceImpl>
{
    let mut impls = vec![];
    append_builtin_impl(&mut impls, builtins.int_id.clone(), builtins.add_id.clone(), None);
    impls
}

fn append_builtin_impl(v: &mut Vec<InterfaceImpl>, implementee: Uuid, interface: Uuid, returned: Option<Uuid>)
{
    let implementee = TypePattern::Primary { id: implementee, generics: vec![] };

    let returned = returned.map(|r| TypePattern::Primary { id: r, generics: vec![] }).unwrap_or(implementee.clone());
    let interface = TypePattern::Primary { id: interface, generics: vec![returned] };
    let b = InterfaceImpl::new_builtin(implementee, interface);
    v.push(b);
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