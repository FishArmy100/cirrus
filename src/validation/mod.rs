pub mod type_error;
pub mod type_def;

pub use type_error::*;
pub use type_def::*;
use uuid::Uuid;

use crate::ast::TypeName;

pub enum TypePattern
{
    Primary
    {
        id: Uuid,
        generics: Vec<TypePattern>,
    },
}

impl TypePattern
{
    pub fn from_ast(type_name: &TypeName, globals: &ProgramTypeDefinitions) -> Result<Self, TypeError>
    {
        match type_name
        {
            TypeName::Identifier { name, args } => {
                let Some(type_def) = globals.get_from_name(&name.value_string().unwrap()) else {
                    return Err(TypeError::UnknownType { name: name.clone(), pos: type_name.get_pos() });
                };

                let generics_count = args.as_ref().map_or(0, |args| args.args.len());

                if type_def.get_generic_count() != generics_count
                {
                    return Err(TypeError::GenericCountMismatch { 
                        expected: type_def.get_generic_count(), 
                        got: generics_count, 
                        pos: type_name.get_pos(), 
                    });
                }
                
                let generics: Result<Vec<_>, _> = args.as_ref()
                    .map_or(vec![], |g| g.args.iter().map(|g| Self::from_ast(g, globals)).collect())
                    .into_iter().collect();
                
                Ok(Self::Primary { id: type_def.get_id().clone(), generics: generics? })
            },
            t => return Err(TypeError::NotSupported { 
                feature: NotSupportedFeature::NonPrimaryImpls, 
                pos: t.get_pos() 
            })
        }
    }
}

pub struct InterfaceImpl
{
    pub implemented: TypePattern,
    pub interface: TypePattern,
}