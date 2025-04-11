use uuid::Uuid;

use crate::{ast::{Declaration, Program}, lexing::token::Token, utils::{partition_errors, TextPos}};

pub enum TypeError
{
    DuplicateTypeDefinition
    {
        original: Token,
        duplicate: Token,
    }
}

pub struct GenericParam 
{
    pub name: String,
}

pub struct StructDef
{
    pub id: Uuid,
    pub name: String,
    pub pos: TextPos,
    pub generic_params: Option<Vec<GenericParam>>
}

impl StructDef
{
    pub fn find_global_defs(program: Program) -> Result<Vec<StructDef>, Vec<TypeError>>
    {
        let results = program.declarations.iter().filter_map(|d| match d {
            Declaration::Struct(_, s) => 
            {
                let struct_id = s.id.value.as_ref().unwrap().as_string().unwrap();
                let generic_params = s.generic_params.as_ref().map(|p| p.params.iter().map(|p| p.value_string().unwrap().clone()));

                if let Some(t) = s.generic_params.as_ref().map(|p| p.params.iter().find(|p| p.value.as_ref().unwrap().as_string().unwrap() == struct_id)).flatten()
                {
                    return Some(Err(TypeError::DuplicateTypeDefinition { original: s.id.clone(), duplicate: t.clone() }))
                }

                Some(Ok(StructDef
                {
                    id: Uuid::new_v4(),
                    name: struct_id.clone(),
                    generic_params: s.generic_params.as_ref().map(|p| p.params.iter().map(|p|{ 
                        let name = p.value_string().unwrap().clone();
                        GenericParam {
                            name,
                        }
                    }).collect())
                }))
            },
            _ => None,
        });

        partition_errors(results)
    }
}